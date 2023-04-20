use serenity::builder::CreateApplicationCommands;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use std::collections::HashMap;
use tracing::log::info;

use crate::bot::commands::MixerCommand;

pub struct MixerCommandHandler {
    commands: HashMap<String, Box<dyn MixerCommand>>,
}

impl MixerCommandHandler {
    pub fn new(commands: HashMap<String, Box<dyn MixerCommand>>) -> Self {
        Self { commands }
    }

    pub fn add_command<T: MixerCommand + 'static>(&mut self, command: T) {
        self.commands.insert(command.name(), Box::new(command));
    }

    pub fn create_all(&self, create_commands: &mut CreateApplicationCommands) {
        self.commands.values().for_each(|command| {
            create_commands.create_application_command(|create_command| {
                command.create(create_command);
                create_command
            });
            info!("Registered command \"{}\"", command.name())
        });
    }

    pub async fn handle_command(
        &self,
        ctx: &Context,
        interaction: ApplicationCommandInteraction,
    ) -> serenity::Result<()> {
        if let Some(command) = self.commands.get(&interaction.data.name) {
            info!(
                "User {} ({}) executed command \"{}\"",
                interaction.user.name,
                interaction.user.id,
                command.name()
            );
            return command.execute(&ctx, interaction).await;
        } else {
            info!(
                "User {} ({}) executed unknown command \"{}\"",
                interaction.user.name, interaction.user.id, interaction.data.name
            );
            interaction
                .create_interaction_response(ctx, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.content("This command does not exist!")
                        })
                })
                .await
                .unwrap();
        }

        Ok(())
    }
}
