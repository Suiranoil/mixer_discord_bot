use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::{
    application_command::ApplicationCommandInteraction, InteractionResponseType,
};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use crate::bot::commands::MixerCommand;
use crate::database::queries::prelude::*;
use crate::database::DatabaseContainer;
use crate::CreatorContainer;

#[derive(Clone)]
pub struct CreatorCommand;

#[async_trait]
impl MixerCommand for CreatorCommand {
    fn name(&self) -> String {
        "creator".to_string()
    }

    fn create(&self, command: &mut CreateApplicationCommand) {
        command
            .name(self.name())
            .description("Hello world!")
            .create_option(|option| {
                option
                    .name("verify")
                    .description("Verify or unverify the server")
                    .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|sub_option| {
                        sub_option
                            .name("verify")
                            .description("Whether to verify or unverify the server")
                            .kind(CommandOptionType::Boolean)
                            .required(true)
                    })
            })
            .dm_permission(false);
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: ApplicationCommandInteraction,
    ) -> serenity::Result<()> {
        let has_permission = {
            let data = ctx.data.read().await;
            let creator = data.get::<CreatorContainer>().unwrap().clone();
            interaction.user.id == *creator
        };

        if !has_permission {
            interaction
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message
                                .content("You are not the creator of this bot!")
                                .ephemeral(true)
                        })
                })
                .await
                .unwrap();
            return Ok(());
        }

        let data = interaction.data.options.get(0).unwrap().clone();
        match data.name.as_str() {
            "verify" => self.process_verify(ctx, interaction.clone(), data).await?,
            _ => {}
        }

        Ok(())
    }
}

impl CreatorCommand {
    async fn process_verify(
        &self,
        ctx: &Context,
        interaction: ApplicationCommandInteraction,
        data: CommandDataOption,
    ) -> serenity::Result<()> {
        let verify = data
            .options
            .get(0)
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .as_bool()
            .unwrap();
        let guild_id = interaction.guild_id.unwrap();

        {
            let data = ctx.data.read().await;
            let db = data.get::<DatabaseContainer>().unwrap().read().await;

            // GuildQuery::create_if_not_exists(db.get_connection(), guild_id).await;
            GuildQuery::set_verified(db.connection(), guild_id, verify).await;
        }

        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message
                            .content(format!("Updated guild verification to {verify}"))
                            .ephemeral(true)
                    })
            })
            .await
            .unwrap();

        Ok(())
    }
}
