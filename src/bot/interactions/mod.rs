use serenity::async_trait;
use serenity::client::Context;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;

#[async_trait]
pub trait MixerInteraction: Sync + Send {
    fn custom_id(&self) -> String;
    // fn create(&self, command: &mut CreateApplicationCommand);
    async fn execute(
        &self,
        ctx: &Context,
        interaction: MessageComponentInteraction,
    ) -> serenity::Result<()>;
}
