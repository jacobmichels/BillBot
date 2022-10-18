use std::{collections::HashMap, process::exit};

use log::{error, info, warn};
use serenity::{
    async_trait,
    model::prelude::{
        command::Command,
        component::ActionRowComponent,
        interaction::{Interaction, InteractionResponseType::ChannelMessageWithSource},
        CommandId, GuildId, Ready,
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
            let command_ids = match register_global_commands(&ctx).await {
                Ok(ids) => ids,
                Err(why) => {
                    error!("FATAL: failed to register a global command: {}", why);
                    exit(1);
                }
            };

            tokio::spawn(async move {
                tokio::signal::ctrl_c()
                    .await
                    .expect("failed to register ctrl-c handler");
                info!("deregistering global interactions...");
                deregister_global_commands(&ctx, command_ids)
                    .await
                    .expect("failed to deregister global interactions");
                info!("done! goodbye");
                exit(0);
            });

            info!("global commands registered")
        } else {
            info!("{:?}", self.guild_ids);

            let command_ids_map = match register_guild_commands(&ctx, &self.guild_ids).await {
                Ok(ids) => ids,
                Err(why) => {
                    error!("FATAL: failed to register a guild command: {}", why);
                    exit(1);
                }
            };

            tokio::spawn(async move {
                tokio::signal::ctrl_c()
                    .await
                    .expect("failed to register ctrl-c handler");
                info!("deregistering guild interactions...");
                deregister_guild_commands(&ctx, command_ids_map)
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
                let mut amount = String::from("");
                let mut method = String::from("");
                let submitter;

                let submitter_nick = submission.user.nick_in(&ctx, guild_id).await;

                if let Some(nick) = submitter_nick {
                    submitter = nick
                } else {
                    warn!("submitter has no nickname, using username");
                    submitter = submission.user.name.clone();
                }

                for row in &submission.data.components {
                    for component in &row.components {
                        match component {
                            ActionRowComponent::InputText(data) => match data.custom_id.as_str() {
                                "name" => title = data.value.clone(),
                                "amount" => amount = data.value.clone(),
                                "method" => method = data.value.clone(),
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

                info!(
                    "submission values, title: {}, amount: {}, method: {}, submitter: {}",
                    title, amount, method, submitter
                );

                if let Err(why) = submission
                    .create_interaction_response(&ctx.http, |res| {
                        res.kind(ChannelMessageWithSource)
                            .interaction_response_data(|msg| msg.content(format!("**🚨 AYO NEW BILL AVAILABLE 🚨**\n >>> Title: {}\nTotal amount: {}\nBill created by: {}\nPayment method: {}\n\n *Thanks lads ❤️*", title, amount, submitter, method)))
                    })
                    .await
                {
                    error!("failed to respond to modal submission: {}", why);
                }
            }
            _ => {}
        }
    }
}

async fn register_global_commands(ctx: &Context) -> anyhow::Result<Vec<CommandId>> {
    let mut command_ids = Vec::with_capacity(2);

    let cmd = Command::create_global_application_command(&ctx.http, |cmd| {
        commands::create_bill::register(cmd)
    })
    .await?;

    command_ids.push(cmd.id);

    let cmd =
        Command::create_global_application_command(&ctx.http, |cmd| commands::help::register(cmd))
            .await?;

    command_ids.push(cmd.id);

    Ok(command_ids)
}

async fn deregister_global_commands(
    ctx: &Context,
    command_ids: Vec<CommandId>,
) -> anyhow::Result<()> {
    // this could be ran concurrently
    for command_id in command_ids {
        Command::delete_global_application_command(&ctx.http, command_id).await?;
    }

    Ok(())
}

async fn register_guild_commands(
    ctx: &Context,
    guild_ids: &Vec<String>,
) -> anyhow::Result<HashMap<String, Vec<CommandId>>> {
    let mut command_ids_map: HashMap<String, Vec<CommandId>> =
        HashMap::with_capacity(guild_ids.len());

    for id in guild_ids {
        let guild = GuildId(id.parse()?);
        let mut command_ids = Vec::with_capacity(2);

        let cmd = guild
            .create_application_command(&ctx.http, |cmd| commands::create_bill::register(cmd))
            .await?;
        command_ids.push(cmd.id);

        let cmd = guild
            .create_application_command(&ctx.http, |cmd| commands::help::register(cmd))
            .await?;
        command_ids.push(cmd.id);

        command_ids_map.insert(id.to_string(), command_ids);
    }

    Ok(command_ids_map)
}

async fn deregister_guild_commands(
    ctx: &Context,
    command_ids_map: HashMap<String, Vec<CommandId>>,
) -> anyhow::Result<()> {
    for (key, value) in command_ids_map {
        let guild = GuildId(key.parse()?);

        for id in value {
            guild.delete_application_command(&ctx.http, id).await?;
        }
    }

    Ok(())
}
