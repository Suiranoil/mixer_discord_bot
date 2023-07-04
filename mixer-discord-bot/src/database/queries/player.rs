use itertools::Itertools;
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};
use serenity::model::prelude::UserId;

use crate::mixer::rating::Rating;
use entity::players;
use entity::prelude::*;

pub struct Query;

impl Query {
    pub async fn create(
        connection: &DatabaseConnection,
        user_id: UserId,
    ) -> Option<players::Model> {
        let player = players::ActiveModel {
            discord_id: Set(user_id.0 as i64),
            ..Default::default()
        };

        Players::insert(player).exec(connection).await.ok()?;

        Self::player_by_user_id(connection, user_id).await
    }

    pub async fn create_if_not_exists(
        connection: &DatabaseConnection,
        user_id: UserId,
    ) -> Option<players::Model> {
        if let Some(player) = Self::player_by_user_id(connection, user_id).await {
            Some(player)
        } else {
            Self::create(connection, user_id).await
        }
    }

    pub async fn player_by_user_id(
        connection: &DatabaseConnection,
        user_id: UserId,
    ) -> Option<players::Model> {
        Players::find()
            .filter(players::Column::DiscordId.eq(user_id.0 as i64))
            .one(connection)
            .await
            .ok()?
    }

    pub async fn players_by_user_ids(
        connection: &DatabaseConnection,
        user_ids: Vec<UserId>,
    ) -> Option<Vec<players::Model>> {
        Players::find()
            .filter(
                players::Column::DiscordId
                    .is_in(user_ids.iter().map(|id| id.0 as i64).collect_vec()),
            )
            .all(connection)
            .await
            .ok()
    }

    pub async fn update_rating(
        connection: &DatabaseConnection,
        user_id: UserId,
        role: Role,
        rating: Rating,
    ) -> Option<players::Model> {
        let mut player = Self::player_by_user_id(connection, user_id)
            .await?
            .into_active_model();

        match role {
            Role::Tank => {
                player.tank_rating = Set(rating.value);
                player.tank_rd = Set(rating.rd);
                player.tank_volatility = Set(rating.volatility);
            }
            Role::Dps => {
                player.dps_rating = Set(rating.value);
                player.dps_rd = Set(rating.rd);
                player.dps_volatility = Set(rating.volatility);
            }
            Role::Support => {
                player.support_rating = Set(rating.value);
                player.support_rd = Set(rating.rd);
                player.support_volatility = Set(rating.volatility);
            }
        }

        Players::update(player).exec(connection).await.ok()
    }

    pub async fn update_preference(
        connection: &DatabaseConnection,
        user_id: UserId,
        flex: bool,
        primary: Option<Role>,
        secondary: Option<Role>,
        tertiary: Option<Role>,
    ) -> Option<players::Model> {
        let mut player = Self::player_by_user_id(connection, user_id)
            .await?
            .into_active_model();

        player.flex = Set(flex);
        player.primary_role = Set(primary);
        player.secondary_role = Set(secondary);
        player.tertiary_role = Set(tertiary);

        Players::update(player).exec(connection).await.ok()
    }

    pub async fn update_last_played(
        connection: &DatabaseConnection,
        user_id: UserId,
        last_played: DateTime,
    ) -> Option<players::Model> {
        let mut player = Self::player_by_user_id(connection, user_id)
            .await?
            .into_active_model();

        player.last_played = Set(Some(last_played));

        Players::update(player).exec(connection).await.ok()
    }
}
