use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::id::ChannelId;
use serenity::model::prelude::{GuildId, VoiceState};
use crate::database::DatabaseContainer;
use crate::bot::handlers::command_handler::MixerCommandHandlerContainer;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        println!("{} is connected!", data_about_bot.user.name);
        let data = ctx.data.read().await;
        let command_handler = data.get::<MixerCommandHandlerContainer>().unwrap();

        Command::set_global_application_commands(&ctx.http, |commands| {
            command_handler.create_all(commands);
            commands
        }).await.unwrap();
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        let data = ctx.data.read().await;
        let db = data.get::<DatabaseContainer>().unwrap().read().await;

        let user_id = new.user_id;
        let new_channel_id = new.channel_id.unwrap_or(ChannelId(0));
        let guild_id = new.guild_id.unwrap_or(GuildId(0));

        match db.get_lobby_by_channel(guild_id, new_channel_id).await {
            Some(_) => {
                if !db.has_player(user_id).await {
                    db.insert_player(user_id).await;
                }

                println!("{} joined lobby", user_id);
            }
            None => {}
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let data = ctx.data.read().await;
        let command_handler = data.get::<MixerCommandHandlerContainer>().unwrap();

        match interaction {
            Interaction::ApplicationCommand(command) => {
                command_handler.handle_command(&ctx, command).await.unwrap();
            }
            _ => {}
        }
    }
}