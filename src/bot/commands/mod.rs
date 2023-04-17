pub mod creator;
pub mod lobby;
pub mod ping;
pub mod preference;
pub mod rating;
pub mod settings;

use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

#[async_trait]
pub trait MixerCommand: Sync + Send {
    fn name(&self) -> String;
    fn create(&self, command: &mut CreateApplicationCommand);
    async fn execute(
        &self,
        ctx: &Context,
        interaction: ApplicationCommandInteraction,
    ) -> serenity::Result<()>;
}
