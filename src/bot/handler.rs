use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use crate::bot::MixerBotContainer;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        println!("{} is connected!", data_about_bot.user.name);

        let data = ctx.data.read().await;
        let bot_commands = &data.get::<MixerBotContainer>().unwrap().read().await.commands;
        Command::set_global_application_commands(&ctx.http, |commands| {
            for cmd in bot_commands.values() {
                commands.create_application_command(|command| {
                    cmd.create(command);
                    command
                });
                println!("Registered command \"{}\"", cmd.name())
            }
            commands
        }).await.unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let data = ctx.data.read().await;
        let bot_commands = &data.get::<MixerBotContainer>().unwrap().read().await.commands;

        match interaction {
            Interaction::ApplicationCommand(command) =>
                if let Some(mixer_command) = bot_commands.get(&command.data.name) {
                    mixer_command.execute(&ctx, command).await.unwrap()
                }
            _ => {}
        }
    }
}