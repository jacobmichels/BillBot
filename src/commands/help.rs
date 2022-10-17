use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("help")
        .description("Display help with commands")
}

pub fn run(_options: &[CommandDataOption]) -> String {
    "TODO".to_string()
}
