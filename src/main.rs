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

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .context("failed to build serenity client")?;

    info!("starting client");
    client.start().await?;

    Ok(())
}
