pub mod ping;

use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::async_trait;

#[async_trait]
pub trait MixerCommand: Sync + Send {
    fn name(&self) -> String;
    fn create(&self, command: &mut CreateApplicationCommand);
    async fn execute(&self, ctx: &Context, interaction: ApplicationCommandInteraction) -> serenity::Result<()>;
}