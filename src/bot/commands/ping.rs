use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::{
    application_command::ApplicationCommandInteraction,
    InteractionResponseType
};
use serenity::async_trait;
use crate::bot::commands::MixerCommand;

#[derive(Clone)]
pub struct Ping;

#[async_trait]
impl MixerCommand for Ping {
    fn name(&self) -> String {
        "ping".to_string()
    }

    fn create(&self, command: &mut CreateApplicationCommand) {
        command.name(self.name()).description("Hello world!");
    }

    async fn execute(&self, ctx: &Context, interaction: ApplicationCommandInteraction) -> serenity::Result<()> {
        let content = "Pong!";
        interaction.create_interaction_response(&ctx.http, |response| {
            response.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content(content)
                })
        }).await?;

        println!("{:#?}", interaction.get_interaction_response(&ctx.http).await?);

        println!("Interacted");

        Ok(())
    }
}