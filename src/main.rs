use std::env;

use anyhow::Context;
use handler::Handler;
use log::info;
use serenity::{prelude::GatewayIntents, Client};

mod commands;
mod handler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let token = env::var("BILLBOT_TOKEN").context("BILLBOT_TOKEN not found")?;
    let guild_ids_str = env::var("BILLBOT_GUILDS").unwrap_or_else(|_| "".to_owned());

    let mut guild_ids: Vec<String> = guild_ids_str.split(',').map(|id| id.to_owned()).collect();
    if guild_ids.first().unwrap() == "" {
        guild_ids.remove(0);
    }

    let mut client = Client::builder(token, GatewayIntents::GUILD_MEMBERS)
        .event_handler(Handler::new(guild_ids))
        .await
        .context("failed to build serenity client")?;

    info!("starting client");
    client.start().await?;

    Ok(())
}
