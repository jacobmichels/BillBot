use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType::ChannelMessageWithSource;
use serenity::prelude::Context;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("help")
        .description("Display help with commands")
}

pub async fn respond(ctx: &Context, cmd: &ApplicationCommandInteraction) -> anyhow::Result<()> {
    cmd.create_interaction_response(&ctx.http, |res| {
        res.kind(ChannelMessageWithSource)
            .interaction_response_data(|msg| msg.content("TODO"))
    })
    .await?;
    Ok(())
}
