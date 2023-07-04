use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};
use serenity::model::prelude::GuildId;

use entity::guilds;
use entity::prelude::*;

pub struct Query;

impl Query {
    pub async fn create(
        connection: &DatabaseConnection,
        guild_id: GuildId,
    ) -> Option<guilds::Model> {
        let guild = guilds::ActiveModel {
            guild_id: Set(guild_id.0 as i64),
            verified: Set(false),
            ..Default::default()
        };

        Guilds::insert(guild).exec(connection).await.ok()?;

        Self::guild_by_guild_id(connection, guild_id).await
    }

    pub async fn create_if_not_exists(
        connection: &DatabaseConnection,
        guild_id: GuildId,
    ) -> Option<guilds::Model> {
        if let Some(guild) = Self::guild_by_guild_id(connection, guild_id).await {
            Some(guild)
        } else {
            Self::create(connection, guild_id).await
        }
    }

    pub async fn guild_by_guild_id(
        connection: &DatabaseConnection,
        guild_id: GuildId,
    ) -> Option<guilds::Model> {
        Guilds::find()
            .filter(guilds::Column::GuildId.eq(guild_id.0 as i64))
            .one(connection)
            .await
            .ok()?
    }

    pub async fn set_verified(
        connection: &DatabaseConnection,
        guild_id: GuildId,
        verified: bool,
    ) -> Option<guilds::Model> {
        let mut guild = Query::guild_by_guild_id(connection, guild_id)
            .await?
            .into_active_model();

        guild.verified = Set(verified);

        Guilds::update(guild).exec(connection).await.ok()
    }
}
