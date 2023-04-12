pub mod commands;
pub mod interactions;
mod handlers;

use std::collections::HashMap;
use serenity::client::{Context, EventHandler};
use serenity::http::CacheHttp;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::prelude::VoiceState;

use crate::bot::commands::MixerCommand;
use crate::bot::handlers::command_handler::MixerCommandHandler;
use crate::database::DatabaseContainer;

pub struct MixerBot {
    command_handler: MixerCommandHandler,
}

impl MixerBot {
    pub fn new() -> Self {
        Self {
            command_handler: MixerCommandHandler::new(HashMap::new())
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
        println!("{} is connected!", data_about_bot.user.name);

        Command::set_global_application_commands(&ctx.http, |commands| {
            self.command_handler.create_all(commands);
            commands
        }).await.unwrap();
    }

    async fn voice_state_update(&self, ctx: Context, _old: Option<VoiceState>, new: VoiceState) {
        if let Some(guild_id) = new.guild_id {
            if let Some(channel_id) = new.channel_id {
                let data = ctx.data.read().await;
                let db = data.get::<DatabaseContainer>().unwrap().read().await;

                if let Some(_) = db.get_lobby_by_channel(guild_id, channel_id).await {
                    if let Some(member) = new.member {
                        if member.user.bot {
                            return;
                        }
                    }

                    if db.get_player(new.user_id).await.is_none() {
                        db.insert_player(new.user_id).await;
                    }
                }
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(command) => {
                self.command_handler.handle_command(&ctx, command).await.unwrap();
            }
            Interaction::MessageComponent(component) => {
                component.create_interaction_response(ctx.http(), |response| {
                    response.kind(InteractionResponseType::DeferredUpdateMessage)
                }).await.unwrap();
            }
            _ => {}
        }
    }
}
