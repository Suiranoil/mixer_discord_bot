use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::CommandDataOptionValue::User;
use serenity::model::application::interaction::{
    application_command::ApplicationCommandInteraction, InteractionResponseType,
};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::Permissions;

use crate::bot::commands::MixerCommand;
use crate::database::models::role::Role;
use crate::database::queries::prelude::*;
use crate::database::DatabaseContainer;
use crate::mixer::rating::Rating;

#[derive(Clone)]
pub struct RatingCommand;

#[async_trait]
impl MixerCommand for RatingCommand {
    fn name(&self) -> String {
        "rating".to_string()
    }

    fn create(&self, command: &mut CreateApplicationCommand) {
        command
            .name(self.name())
            .description("Set user's rating on specific role")
            .create_option(|option| {
                option
                    .name("set")
                    .description("Set a user's rank")
                    .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|option| {
                        option
                            .name("user")
                            .description("The user to set the rank for")
                            .kind(CommandOptionType::User)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("role")
                            .description("The role to set the rank for")
                            .kind(CommandOptionType::String)
                            .required(true)
                            .add_string_choice("tank", "tank")
                            .add_string_choice("dps", "dps")
                            .add_string_choice("support", "support")
                    })
                    .create_sub_option(|option| {
                        option
                            .name("rank")
                            .description("The rank to set")
                            .kind(CommandOptionType::Integer)
                            .required(true)
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
        match interaction.data.options.get(0).unwrap().name.as_str() {
            "set" => {
                let user = match interaction
                    .data
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
                            .create_interaction_response(&ctx.http, |response| {
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

                let role = Role::try_from(
                    interaction
                        .data
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
                .ok()
                .unwrap();
                let rating = interaction
                    .data
                    .options
                    .get(0)
                    .unwrap()
                    .options
                    .get(2)
                    .unwrap()
                    .value
                    .as_ref()
                    .unwrap()
                    .as_u64()
                    .unwrap();

                if rating < 1 || rating > 5000 {
                    interaction
                        .create_interaction_response(ctx, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    message
                                        .content(format!("Rank must be between 1 and 5000"))
                                        .ephemeral(true)
                                })
                        })
                        .await?;
                    return Ok(());
                }

                {
                    let data = ctx.data.read().await;
                    let db = data.get::<DatabaseContainer>().unwrap().read().await;

                    PlayerQuery::update_rating(
                        db.connection(),
                        user.id,
                        role,
                        Rating::new_no_sigma(rating as f32, 125.0),
                    )
                    .await;
                }

                interaction
                    .create_interaction_response(ctx, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message
                                    .content(format!(
                                        "Setting rank for user <@{}> to {} {}",
                                        user.id,
                                        String::from(role),
                                        rating
                                    ))
                                    .allowed_mentions(|mentions| mentions.empty_users())
                                    .ephemeral(true)
                            })
                    })
                    .await?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
