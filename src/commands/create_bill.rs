use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::component::InputTextStyle::{Paragraph, Short};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType::Modal;
use serenity::prelude::Context;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("bill")
        .description("Manage bills")
        .create_option(|opt| {
            opt.kind(CommandOptionType::SubCommand)
                .name("create")
                .description("Create a new bill, which billbot will post to the server")
        })
        .create_option(|opt| {
            opt.kind(CommandOptionType::SubCommand)
                .name("paid")
                .description("Mark an existing bill as paid")
        })
}

pub async fn respond(ctx: &Context, cmd: &ApplicationCommandInteraction) -> anyhow::Result<()> {
    cmd.create_interaction_response(&ctx.http, |res| {
        res.kind(Modal).interaction_response_data(|modal| {
            modal
                .custom_id("bill_create_modal")
                .title("Create a new bill")
                .components(|cmp| {
                    cmp.create_action_row(|row| {
                        row.create_input_text(|input| {
                            input
                                .custom_id("name")
                                .label("Bill Name")
                                .style(Short)
                                .placeholder("dons run")
                        })
                    });
                    cmp.create_action_row(|row| {
                        row.create_input_text(|input| {
                            input
                                .custom_id("amount")
                                .label("Amount")
                                .style(Short)
                                .placeholder("420.69")
                        })
                    });
                    cmp.create_action_row(|row| {
                        row.create_input_text(|input| {
                            input
                                .custom_id("method")
                                .label("Payment method")
                                .style(Paragraph)
                                .placeholder("Ex. etransfer jacob.michels2025@gmail.com")
                        })
                    })
                })
        })
    })
    .await?;

    Ok(())
}
