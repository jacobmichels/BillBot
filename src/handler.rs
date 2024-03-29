use std::{collections::HashMap, process::exit};

use log::{error, info, warn};
use rust_decimal::prelude::*;
use rusty_money::{
    iso::{self},
    Money,
};
use serenity::{
    async_trait,
    model::prelude::{
        command::Command,
        component::ActionRowComponent,
        interaction::{Interaction, InteractionResponseType::ChannelMessageWithSource},
        GuildId, Member, Ready,
    },
    prelude::*,
};

use crate::commands;

pub struct Handler {
    guild_ids: Vec<String>,
}

impl Handler {
    pub fn new(guild_ids: Vec<String>) -> Handler {
        Handler { guild_ids }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, bot_info: Ready) {
        info!("{} is connected", bot_info.user.name);

        // register commands globally if guild_ids is empty
        if self.guild_ids.is_empty() {
            if let Err(why) = register_global_commands(&ctx).await {
                error!("FATAL: failed to register a global command: {}", why);
                exit(1);
            };

            tokio::spawn(async move {
                tokio::signal::ctrl_c()
                    .await
                    .expect("failed to register ctrl-c handler");
                info!("deregistering global interactions...");
                deregister_global_commands(&ctx)
                    .await
                    .expect("failed to deregister global interactions");
                info!("done! goodbye");
                exit(0);
            });

            info!("global commands registered")
        } else {
            info!("supplied guild_ids: {:?}", self.guild_ids);

            if let Err(why) = register_guild_commands(&ctx, &self.guild_ids).await {
                error!("FATAL: failed to register a guild command: {}", why);
                exit(1);
            };

            let guild_ids = self.guild_ids.clone();
            tokio::spawn(async move {
                tokio::signal::ctrl_c()
                    .await
                    .expect("failed to register ctrl-c handler");
                info!("deregistering guild interactions...");
                deregister_guild_commands(&ctx, &guild_ids)
                    .await
                    .expect("failed to deregister guild interactions");
                info!("done! goodbye");
                exit(0);
            });

            info!("guild commands registered")
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(command) => {
                info!(
                    "received command interaction, name: {}, interaction ID: {}",
                    command.data.name, command.id
                );

                let interaction_response_result = match command.data.name.as_str() {
                    "bill" => commands::create_bill::respond(&ctx, &command).await,
                    _ => commands::help::respond(&ctx, &command).await,
                };

                if let Err(why) = interaction_response_result {
                    error!("failed to respond to interaction: {}", why);
                }
                info!(
                    "responded to command interaction, name: {}, interaction ID: {}",
                    command.data.name, command.id
                );
            }
            Interaction::ModalSubmit(submission) => {
                info!("received modal submit interaction");

                let guild_id = submission.guild_id;
                if guild_id.is_none() {
                    error!("no guild id for modal submit, aborting response");
                    return;
                }

                let guild_id = guild_id.unwrap();

                let mut title = String::from("");
                let mut amount = Money::from_major(10, iso::CAD);
                let mut method = String::from("");
                let mut payers: Vec<Member> = Vec::with_capacity(10);
                let submitter;

                let submitter_nick = submission.user.nick_in(&ctx, guild_id).await;

                if let Some(nick) = submitter_nick {
                    submitter = nick.trim().to_owned()
                } else {
                    warn!("submitter has no nickname, using username");
                    submitter = submission.user.name.clone().trim().to_owned();
                }

                for row in &submission.data.components {
                    for component in &row.components {
                        match component {
                            ActionRowComponent::InputText(data) => match data.custom_id.as_str() {
                                "name" => title = data.value.clone().trim().to_owned(),
                                "amount" => {
                                    let amount_str = data.value.trim();
                                    let cad = Money::from_str(amount_str, iso::CAD);
                                    if let Err(why) = cad {
                                        error!("failed to convert {} to CAD: {}", amount_str, why);
                                        if let Err(why) = submission
                                            .create_interaction_response(&ctx.http, |res| {
                                                res.kind(ChannelMessageWithSource)
                                                    .interaction_response_data(|msg| {
                                                        msg.ephemeral(true).content(format!(
                                                            "{} is not a valid CAD amount",
                                                            amount_str
                                                        ))
                                                    })
                                            })
                                            .await
                                        {
                                            error!(
                                                "failed to send early interaction response: {}",
                                                why
                                            );
                                            return;
                                        }
                                        return;
                                    }
                                    amount = cad.unwrap();
                                }
                                "method" => method = data.value.clone().trim().to_owned(),
                                "payers" => {
                                    let payer_string = data.value.clone();
                                    let payer_nicks: Vec<String> = payer_string
                                        .split(',')
                                        .map(|s| s.trim().to_owned())
                                        .collect();
                                    info!("payer nicks: {:?}", payer_nicks);

                                    if submission.guild_id.is_none() {
                                        error!(
                                            "no guild_id for modal submission, aborting response"
                                        );
                                        return;
                                    }

                                    let guild_id = submission.guild_id.unwrap();
                                    let guild_members =
                                        guild_id.members(&ctx.http, None, None).await;
                                    if let Err(why) = guild_members {
                                        error!("failed to get members for guild, aborting response: {}", why);
                                        return;
                                    }
                                    let guild_members = guild_members.unwrap();

                                    let mut channel_members: Vec<Member> =
                                        Vec::with_capacity(guild_members.len());

                                    let channel_id = submission.channel_id;
                                    let channels = guild_id.channels(&ctx.http).await;
                                    if let Err(why) = channels {
                                        error!("failed to list channels in guild: {}", why);
                                        return;
                                    }

                                    let channels = channels.unwrap();
                                    let channel = channels.get(&channel_id);
                                    if channel.is_none() {
                                        error!("submission channel does not exist in guild",);
                                        return;
                                    }

                                    let channel = channel.unwrap();

                                    for guild_member in &guild_members {
                                        let perms = channel
                                            .permissions_for_user(&ctx, guild_member.user.id);
                                        if let Err(why) = perms {
                                            error!("failed to get perms for a user: {}", why);
                                            return;
                                        }

                                        if perms.unwrap().view_channel() {
                                            channel_members.push(guild_member.clone());
                                        }
                                    }

                                    let mut members = HashMap::with_capacity(channel_members.len());

                                    for member in channel_members {
                                        let name = member
                                            .nick
                                            .clone()
                                            .unwrap_or_else(|| member.user.name.clone());

                                        if members.contains_key(&name) {
                                            warn!("two members with same name found in guild, name: {}", name);
                                            continue;
                                        }

                                        members.insert(name, member.clone());
                                    }

                                    for payer in payer_nicks {
                                        let found_member = members.get(&payer);
                                        if found_member.is_none() {
                                            warn!("payer not found, returning early: {}", payer);
                                            if let Err(why) = submission
                                                .create_interaction_response(&ctx.http, |res| {
                                                    res.kind(ChannelMessageWithSource)
                                                        .interaction_response_data(|msg| {
                                                            msg.ephemeral(true).content(format!(
                                                                "Payer not found: {}",
                                                                payer
                                                            ))
                                                        })
                                                })
                                                .await
                                            {
                                                error!("failed to send early response: {}", why);
                                            }
                                            return;
                                        }
                                        payers.push(found_member.unwrap().clone())
                                    }
                                }
                                _ => {
                                    error!("invalid input text custom_id")
                                }
                            },
                            _ => {
                                error!("invalid compenent type");
                            }
                        }
                    }
                }

                let each_owes = Money::from_decimal(
                    amount
                        .amount()
                        .checked_div(Decimal::new(payers.len().try_into().unwrap(), 0))
                        .unwrap(),
                    iso::CAD,
                );

                info!(
                    "submission values, title: {}, amount: {}, method: {}, submitter: {}, payer count: {} each pays: {}",
                    title, amount.to_string(), method, submitter, payers.len(), each_owes
                );

                let payer_mentions = create_payer_mention_string(&payers);

                if let Err(why) = submission
                    .create_interaction_response(&ctx.http, |res| {
                        res.kind(ChannelMessageWithSource)
                            .interaction_response_data(|msg| msg.content(format!("**🚨 AYO NEW BILL AVAILABLE 🚨**\n >>> Title: {}\nTotal amount: {}\nBill created by: {}\nPayment method: {}\nPayers: {}\nEach pays: {}\n\n *Thanks lads ❤️*", title, amount, submitter, method, payer_mentions, each_owes)))
                    })
                    .await
                {
                    error!("failed to respond to modal submission: {}", why);
                }
            }
            Interaction::Autocomplete(_autocomplete) => {
                info!("received autocomplete interaction");
            }
            _ => {}
        }
    }
}

async fn register_global_commands(ctx: &Context) -> anyhow::Result<()> {
    Command::create_global_application_command(&ctx.http, |cmd| {
        commands::create_bill::register(cmd)
    })
    .await?;

    Command::create_global_application_command(&ctx.http, |cmd| commands::help::register(cmd))
        .await?;

    Ok(())
}

async fn deregister_global_commands(ctx: &Context) -> anyhow::Result<()> {
    let commands = Command::get_global_application_commands(&ctx.http).await?;

    // this could be ran concurrently
    for command in commands {
        Command::delete_global_application_command(&ctx.http, command.id).await?;
    }

    Ok(())
}

async fn register_guild_commands(ctx: &Context, guild_ids: &Vec<String>) -> anyhow::Result<()> {
    for id in guild_ids {
        let guild = GuildId(id.parse()?);

        guild
            .create_application_command(&ctx.http, |cmd| commands::create_bill::register(cmd))
            .await?;

        guild
            .create_application_command(&ctx.http, |cmd| commands::help::register(cmd))
            .await?;
    }

    Ok(())
}

async fn deregister_guild_commands(ctx: &Context, guild_ids: &Vec<String>) -> anyhow::Result<()> {
    for guild_id in guild_ids {
        let guild = GuildId(guild_id.parse()?);

        let commands = guild.get_application_commands(&ctx.http).await?;
        for command in commands {
            guild
                .delete_application_command(&ctx.http, command.id)
                .await?;
        }
    }

    Ok(())
}

#[allow(clippy::single_char_add_str)]
fn create_payer_mention_string(payers: &Vec<Member>) -> String {
    let mut s = String::from("");

    for payer in payers {
        s.push_str(payer.mention().to_string().as_str());
        s.push_str(" ");
    }

    s.trim_end().to_owned()
}
