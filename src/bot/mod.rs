pub mod commands;
mod handlers;
pub mod interactions;

use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::http::CacheHttp;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::prelude::VoiceState;
use std::collections::HashMap;
use tracing::log::info;

use crate::bot::commands::MixerCommand;
use crate::bot::handlers::command_handler::MixerCommandHandler;
use crate::database::queries::prelude::*;
use crate::database::DatabaseContainer;
use crate::CreatorContainer;

pub struct MixerBot {
    command_handler: MixerCommandHandler,
}

impl MixerBot {
    pub fn new() -> Self {
        Self {
            command_handler: MixerCommandHandler::new(HashMap::new()),
        }
    }

    pub fn add_command<T: MixerCommand + 'static>(&mut self, command: T) -> &mut Self {
        self.command_handler.add_command(command);
        self
    }
}

#[async_trait]
impl EventHandler for MixerBot {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        info!("{} is connected!", data_about_bot.user.name);

        Command::set_global_application_commands(&ctx.http, |commands| {
            self.command_handler.create_all(commands);
            commands
        })
        .await
        .unwrap();
    }

    async fn voice_state_update(&self, ctx: Context, _old: Option<VoiceState>, new: VoiceState) {
        if let Some(guild_id) = new.guild_id {
            if let Some(channel_id) = new.channel_id {
                let data = ctx.data.read().await;
                let db = data.get::<DatabaseContainer>().unwrap().read().await;

                if let Some(_) =
                    LobbyQuery::lobby_by_channel_id(db.connection(), guild_id, channel_id).await
                {
                    if let Some(member) = new.member {
                        if member.user.bot {
                            return;
                        }
                    }

                    PlayerQuery::create_if_not_exists(db.connection(), new.user_id).await;
                }
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(command) => {
                let has_permission = {
                    let data = ctx.data.read().await;
                    let creator = data.get::<CreatorContainer>().unwrap().clone();
                    let db = data.get::<DatabaseContainer>().unwrap().read().await;
                    let guild = GuildQuery::create_if_not_exists(
                        db.connection(),
                        command.guild_id.unwrap(),
                    )
                    .await;

                    let verified = match guild {
                        Some(guild) => guild.verified,
                        _ => false,
                    };

                    verified || command.user.id == *creator
                };

                if !has_permission {
                    command
                        .create_interaction_response(ctx, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    message
                                        .content("You do not have permission to use this bot!")
                                        .ephemeral(true)
                                })
                        })
                        .await
                        .unwrap();
                    return;
                }

                self.command_handler
                    .handle_command(&ctx, command)
                    .await
                    .unwrap();
            }
            Interaction::MessageComponent(component) => {
                component
                    .create_interaction_response(ctx.http(), |response| {
                        response.kind(InteractionResponseType::DeferredUpdateMessage)
                    })
                    .await
                    .unwrap();
            }
            _ => {}
        }
    }
}
