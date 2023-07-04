use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use serenity::model::prelude::*;

use entity::lobbies;
use entity::prelude::*;

pub struct Query;

impl Query {
    pub async fn create(
        connection: &DatabaseConnection,
        guild_id: GuildId,
        main_voice_id: ChannelId,
        red_team_voice_id: ChannelId,
        blue_team_voice_id: ChannelId,
    ) -> Option<lobbies::Model> {
        let lobby = lobbies::ActiveModel {
            guild_id: Set(guild_id.0 as i64),
            main_voice_id: Set(main_voice_id.0 as i64),
            red_team_voice_id: Set(red_team_voice_id.0 as i64),
            blue_team_voice_id: Set(blue_team_voice_id.0 as i64),
            ..Default::default()
        };

        Lobbies::insert(lobby).exec(connection).await.ok()?;

        Self::lobby_by_channel_id(connection, guild_id, main_voice_id).await
    }

    pub async fn lobby_by_channel_id(
        connection: &DatabaseConnection,
        guild_id: GuildId,
        channel_id: ChannelId,
    ) -> Option<lobbies::Model> {
        Lobbies::find()
            .filter(
                lobbies::Column::GuildId.eq(guild_id.0 as i64).and(
                    lobbies::Column::MainVoiceId
                        .eq(channel_id.0 as i64)
                        .or(lobbies::Column::RedTeamVoiceId.eq(channel_id.0 as i64))
                        .or(lobbies::Column::BlueTeamVoiceId.eq(channel_id.0 as i64)),
                ),
            )
            .one(connection)
            .await
            .ok()?
    }
}
