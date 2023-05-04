use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};
use serenity::model::prelude::GuildId;

use crate::database::models::*;

pub struct Query;

impl Query {
    pub async fn create(
        connection: &DatabaseConnection,
        guild_id: GuildId,
    ) -> Option<guild::Model> {
        let guild = guild::ActiveModel {
            guild_id: Set(guild_id.0 as i64),
            verified: Set(false),
            ..Default::default()
        };

        guild::Entity::insert(guild).exec(connection).await.ok()?;

        Self::guild_by_guild_id(connection, guild_id).await
    }

    pub async fn create_if_not_exists(
        connection: &DatabaseConnection,
        guild_id: GuildId,
    ) -> Option<guild::Model> {
        if let Some(guild) = Self::guild_by_guild_id(connection, guild_id).await {
            Some(guild)
        } else {
            Self::create(connection, guild_id).await
        }
    }

    pub async fn guild_by_guild_id(
        connection: &DatabaseConnection,
        guild_id: GuildId,
    ) -> Option<guild::Model> {
        guild::Entity::find()
            .filter(guild::Column::GuildId.eq(guild_id.0 as i64))
            .one(connection)
            .await
            .ok()?
    }

    pub async fn set_verified(
        connection: &DatabaseConnection,
        guild_id: GuildId,
        verified: bool,
    ) -> Option<guild::Model> {
        let mut guild = Query::guild_by_guild_id(connection, guild_id)
            .await?
            .into_active_model();

        guild.verified = Set(verified);

        guild::Entity::update(guild).exec(connection).await.ok()
    }
}
