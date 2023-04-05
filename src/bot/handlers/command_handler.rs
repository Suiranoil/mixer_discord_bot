use std::collections::HashMap;
use std::sync::Arc;
use serenity::builder::CreateApplicationCommands;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::TypeMapKey;
use crate::bot::commands::MixerCommand;

pub struct MixerCommandHandler {
    commands: HashMap<String, Box<dyn MixerCommand>>,
}

impl MixerCommandHandler {
    pub fn new(commands: HashMap<String, Box<dyn MixerCommand>>) -> Self {
        Self {
            commands
        }
    }

    pub fn create_all(&self, create_commands: &mut CreateApplicationCommands) {
        self.commands.values().for_each(|command| {
            create_commands.create_application_command(|create_command| {
                command.create(create_command);
                create_command
            });
            println!("Registered command \"{}\"", command.name())
        });
    }

    pub async fn handle_command(&self, ctx: &Context, interaction: ApplicationCommandInteraction) -> serenity::Result<()> {
        if let Some(command) = self.commands.get(&interaction.data.name) {
            return command.execute(&ctx, interaction).await
        }

        Ok(())
    }
}

pub struct MixerCommandHandlerContainer;

impl TypeMapKey for MixerCommandHandlerContainer {
    type Value = Arc<MixerCommandHandler>;
}