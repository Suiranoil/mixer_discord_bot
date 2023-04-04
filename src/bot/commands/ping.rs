use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::{
    application_command::ApplicationCommandInteraction,
    InteractionResponseType
};
use serenity::async_trait;
use serenity::http::CacheHttp;
use crate::bot::commands::MixerCommand;

#[derive(Clone)]
pub struct PingCommand;

#[async_trait]
impl MixerCommand for PingCommand {
    fn name(&self) -> String {
        "ping".to_string()
    }

    fn create(&self, command: &mut CreateApplicationCommand) {
        command.name(self.name()).description("Hello world!");
    }

    async fn execute(&self, ctx: &Context, interaction: ApplicationCommandInteraction) -> serenity::Result<()> {
        let content = "Pong!";
        interaction.create_interaction_response(&ctx.http(), |response| {
            response.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content(content).ephemeral(true)
                })
        }).await?;

        let follow1 = interaction.create_followup_message(&ctx.http(), |followup| {
            followup.content("followup1").ephemeral(true)
        }).await?;
        let follow2 = interaction.create_followup_message(&ctx.http(), |followup| {
            followup.content("followup2").ephemeral(true)
        }).await?;

        interaction.delete_followup_message(&ctx.http(), follow1).await?;
        interaction.delete_followup_message(&ctx.http(), follow2).await?;

        println!("{:#?}", interaction.get_interaction_response(&ctx.http).await?);
        // interaction.

        println!("Interacted");

        Ok(())
    }
}