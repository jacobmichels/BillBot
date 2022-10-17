use std::process::exit;

use log::{error, info};
use serenity::{
    async_trait,
    model::prelude::{
        command::Command,
        interaction::{Interaction, InteractionResponseType},
        Ready,
    },
    prelude::*,
};

use crate::commands;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, bot_info: Ready) {
        info!("{} is connected", bot_info.user.name);

        if let Err(why) = register_global_commands(ctx).await {
            error!("FATAL: failed to register a global command: {}", why);
            exit(1);
        }

        info!("commands registered")
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            info!(
                "received command interaction, name: {}, interaction ID: {}",
                command.data.name, command.id
            );

            let response_content = match command.data.name.as_str() {
                "help" => commands::help::run(&command.data.options),
                "bill" => commands::create_bill::run(&command.data.options),
                _ => "command not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |res| {
                    res.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| msg.content(response_content))
                })
                .await
            {
                error!("failed to respond to interaction: {}", why);
            }
            info!(
                "responded to command interaction, name: {}, interaction ID: {}",
                command.data.name, command.id
            );
        }
    }
}

async fn register_global_commands(ctx: Context) -> anyhow::Result<()> {
    Command::create_global_application_command(&ctx.http, |cmd| {
        commands::create_bill::register(cmd)
    })
    .await?;

    Command::create_global_application_command(&ctx.http, |cmd| commands::help::register(cmd))
        .await?;

    Ok(())
}
