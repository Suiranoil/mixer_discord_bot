use sea_orm::entity::prelude::*;
use sea_orm::Iterable;
use serenity::model::id::UserId;
use sqlx::types::chrono::Utc;
use std::collections::HashMap;

use crate::mixer::rating::Rating;
use entity::players;
use entity::prelude::Role;

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub id: i32,
    pub discord_id: UserId,
    pub bn_name: Option<String>,
    pub bn_tag: Option<String>,
    pub last_played: Option<DateTime>,

    pub ranks: HashMap<Role, Rating>,
    pub flex: bool,
    pub priority_roles: Vec<Option<Role>>,
}

impl Player {
    pub fn new(model: players::Model) -> Self {
        Self {
            id: model.id,
            discord_id: UserId::from(model.discord_id as u64),
            bn_name: model.bn_name,
            bn_tag: model.bn_tag,
            last_played: model.last_played,

            ranks: vec![
                (
                    Role::Tank,
                    Rating::new(model.tank_rating, model.tank_rd, model.tank_volatility),
                ),
                (
                    Role::Dps,
                    Rating::new(model.dps_rating, model.dps_rd, model.dps_volatility),
                ),
                (
                    Role::Support,
                    Rating::new(
                        model.support_rating,
                        model.support_rd,
                        model.support_volatility,
                    ),
                ),
            ]
            .into_iter()
            .collect(),

            flex: model.flex,
            priority_roles: vec![
                model.primary_role,
                model.secondary_role,
                model.tertiary_role,
            ],
        }
    }

    pub fn base_priority(&self) -> HashMap<Role, f32> {
        let mut priorities = HashMap::new();
        let time = self.last_played.unwrap_or(Utc::now().naive_utc());
        let time_since = (Utc::now().naive_utc() - time).num_minutes() as f32;
        let priority_points = 100.0 + (time_since / 5.0).powf(1.5);

        if self.flex {
            let role_count = Role::iter().count();
            for role in Role::iter() {
                priorities.insert(role, 1.5 * (priority_points / role_count as f32));
            }

            return priorities;
        }

        for (i, role) in self.priority_roles.iter().enumerate() {
            if let Some(role) = role {
                priorities.insert(*role, priority_points / (i + 1) as f32);
            }
        }

        priorities
    }
}
