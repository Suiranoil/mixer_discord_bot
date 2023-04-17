use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::{
    application_command::ApplicationCommandInteraction, InteractionResponseType,
};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue::User;
use serenity::model::Permissions;

use crate::bot::commands::MixerCommand;
use crate::database::models::role::Role;
use crate::database::queries::prelude::*;
use crate::database::DatabaseContainer;

#[derive(Clone)]
pub struct PreferenceCommand;

#[async_trait]
impl MixerCommand for PreferenceCommand {
    fn name(&self) -> String {
        "preference".to_string()
    }

    fn create(&self, command: &mut CreateApplicationCommand) {
        command
            .name(self.name())
            .description("Hello world!")
            .create_option(|option| {
                option
                    .name("set")
                    .description("Set role preference for user")
                    .kind(CommandOptionType::SubCommandGroup)
                    .create_sub_option(|option| {
                        option
                            .name("flex")
                            .description("Set role preference for user")
                            .kind(CommandOptionType::SubCommand)
                            .create_sub_option(|option| {
                                option
                                    .name("user")
                                    .description("User to set preference for")
                                    .kind(CommandOptionType::User)
                                    .required(true)
                            })
                    })
                    .create_sub_option(|option| {
                        option
                            .name("complex")
                            .description("Set role preference for user")
                            .kind(CommandOptionType::SubCommand)
                            .create_sub_option(|option| {
                                option
                                    .name("user")
                                    .description("User to set preference for")
                                    .kind(CommandOptionType::User)
                                    .required(true)
                            })
                            .create_sub_option(|option| {
                                option
                                    .name("first")
                                    .description("First role preference")
                                    .kind(CommandOptionType::String)
                                    .required(true)
                                    .add_string_choice("Tank", "tank")
                                    .add_string_choice("DPS", "dps")
                                    .add_string_choice("Support", "support")
                                    .add_string_choice("None", "none")
                            })
                            .create_sub_option(|option| {
                                option
                                    .name("second")
                                    .description("Second role preference")
                                    .kind(CommandOptionType::String)
                                    .required(true)
                                    .add_string_choice("Tank", "tank")
                                    .add_string_choice("DPS", "dps")
                                    .add_string_choice("Support", "support")
                                    .add_string_choice("None", "none")
                            })
                            .create_sub_option(|option| {
                                option
                                    .name("third")
                                    .description("Third role preference")
                                    .kind(CommandOptionType::String)
                                    .required(true)
                                    .add_string_choice("Tank", "tank")
                                    .add_string_choice("DPS", "dps")
                                    .add_string_choice("Support", "support")
                                    .add_string_choice("None", "none")
                            })
                    })
            })
            .default_member_permissions(Permissions::ADMINISTRATOR)
            .dm_permission(false);
    }

    async fn execute(
        &self,
        ctx: &Context,
        interaction: ApplicationCommandInteraction,
    ) -> serenity::Result<()> {
        let user = match interaction
            .data
            .options
            .get(0)
            .unwrap()
            .options
            .get(0)
            .unwrap()
            .options
            .get(0)
            .unwrap()
            .resolved
            .as_ref()
            .unwrap()
        {
            User(user, _) => user,
            _ => {
                interaction
                    .create_interaction_response(ctx, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.content(format!("User not found")).ephemeral(true)
                            })
                    })
                    .await?;
                return Ok(());
            }
        };

        match interaction.data.options.get(0).unwrap().name.as_str() {
            "set" => {
                let data = ctx.data.read().await;
                let db = data.get::<DatabaseContainer>().unwrap().read().await;

                match interaction
                    .data
                    .options
                    .get(0)
                    .unwrap()
                    .options
                    .get(0)
                    .unwrap()
                    .name
                    .as_str()
                {
                    "flex" => {
                        PlayerQuery::update_preference(
                            db.connection(),
                            user.id,
                            true,
                            None,
                            None,
                            None,
                        )
                        .await;
                    }
                    "complex" => {
                        let role1 = Role::try_from(
                            interaction
                                .data
                                .options
                                .get(0)
                                .unwrap()
                                .options
                                .get(0)
                                .unwrap()
                                .options
                                .get(1)
                                .unwrap()
                                .value
                                .as_ref()
                                .unwrap()
                                .as_str()
                                .unwrap(),
                        )
                        .ok();
                        let role2 = Role::try_from(
                            interaction
                                .data
                                .options
                                .get(0)
                                .unwrap()
                                .options
                                .get(0)
                                .unwrap()
                                .options
                                .get(2)
                                .unwrap()
                                .value
                                .as_ref()
                                .unwrap()
                                .as_str()
                                .unwrap(),
                        )
                        .ok();
                        let role3 = Role::try_from(
                            interaction
                                .data
                                .options
                                .get(0)
                                .unwrap()
                                .options
                                .get(0)
                                .unwrap()
                                .options
                                .get(3)
                                .unwrap()
                                .value
                                .as_ref()
                                .unwrap()
                                .as_str()
                                .unwrap(),
                        )
                        .ok();

                        PlayerQuery::update_preference(
                            db.connection(),
                            user.id,
                            false,
                            role1,
                            role2,
                            role3,
                        )
                        .await;
                    }
                    _ => {}
                }

                interaction
                    .create_interaction_response(ctx, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message
                                    .content(format!("Preference set for {}", user.name))
                                    .ephemeral(true)
                            })
                    })
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
