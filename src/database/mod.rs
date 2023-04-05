pub mod models;

use std::sync::Arc;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, Database, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};
use sea_orm::ActiveValue::Set;
use serenity::model::id::{ChannelId, UserId};
use serenity::model::prelude::GuildId;
use serenity::prelude::TypeMapKey;
use tokio::fs;
use tokio::sync::RwLock;
use crate::mixer::role::Role;

pub struct MixerDatabase {
    connection: DatabaseConnection
}

impl MixerDatabase {
    pub async fn new(url: &str) -> Self {
        Self {
            connection: Database::connect(url).await.expect("Could not connect to database")
        }
    }

    pub async fn init(&self, script_path: &str) {
        let script = fs::read_to_string(script_path).await.expect("Provided script path for creating database does not exist");

        self.connection.execute_unprepared(&script).await.expect("Invalid database creating script provided");
    }

    pub fn get_connection(&self) -> DatabaseConnection {
        self.connection.clone()
    }

    //CREATE TABLE IF NOT EXISTS players (
    //     id INTEGER PRIMARY KEY AUTOINCREMENT,
    //     discord_id INTEGER NOT NULL,
    //     bn_name TEXT,
    //     bn_tag TEXT,
    //     tank REAL NOT NULL,
    //     dps REAL NOT NULL,
    //     support REAL NOT NULL,
    //     UNIQUE (discord_id)
    // );
    pub async fn insert_player(&self, id: UserId) {
        let player = models::player::ActiveModel {
            discord_id: Set(id.0 as i64),
            bn_name: Set("".to_string()),
            bn_tag: Set("".to_string()),
            ..Default::default()
        };

        player.insert(&self.connection)
            .await
            .expect("Could not insert player into database");
    }

    pub async fn update_player_rank(&self, id: UserId, role: Option<Role>, rank: f32) {
        if !self.has_player(id).await {
            self.insert_player(id).await;
        }

        let mut player = self.get_player(id).await.unwrap().into_active_model();

        match role {
            Some(Role::Tank) => player.tank = Set(rank),
            Some(Role::Dps) => player.dps = Set(rank),
            Some(Role::Support) => player.support = Set(rank),
            None => return
        }

        player.update(&self.connection)
            .await
            .expect("Could not update player rank in database");
    }

    pub async fn update_player_preference(&self, id: UserId, flex: bool, primary: Option<Role>, secondary: Option<Role>, tertiary: Option<Role>) {
        if !self.has_player(id).await {
            self.insert_player(id).await;
        }

        let mut player = self.get_player(id).await.unwrap().into_active_model();

        player.flex = Set(flex);
        player.primary_role = Set(Role::option_to_i32(primary));
        player.secondary_role = Set(Role::option_to_i32(secondary));
        player.tertiary_role = Set(Role::option_to_i32(tertiary));

        player.update(&self.connection)
            .await
            .expect("Could not update player preference in database");
    }

    pub async fn get_player(&self, id: UserId) -> Option<models::player::Model> {
        models::player::Entity::find()
            .filter(models::player::Column::DiscordId.eq(id.0))
            .one(&self.connection)
            .await
            .expect("Could not get player from database")
    }

    pub async fn get_players(&self, ids: Vec<UserId>) -> Vec<models::player::Model> {
        models::player::Entity::find()
            .filter(models::player::Column::DiscordId.is_in(ids.iter().map(|id| id.0).collect::<Vec<u64>>()))
            .all(&self.connection)
            .await
            .expect("Could not get players from database")
    }

    pub async fn has_player(&self, id: UserId) -> bool {
        models::player::Entity::find()
            .filter(models::player::Column::DiscordId.eq(id.0))
            .one(&self.connection)
            .await
            .expect("Could not get player from database")
            .is_some()
    }

    pub async fn get_all_guild_lobbies(&self, guild_id: GuildId) -> Vec<models::lobby::Model> {
        models::lobby::Entity::find()
            .filter(models::lobby::Column::GuildId.eq(guild_id.0))
            .all(&self.connection)
            .await
            .expect("Could not get lobbies from database")
    }

    pub async fn get_guild_lobby(&self, lobby_id: i32) -> Option<models::lobby::Model> {
        models::lobby::Entity::find_by_id(lobby_id)
            .one(&self.connection)
            .await
            .expect("Could not get lobby from database")
    }

    pub async fn insert_guild_lobby(&self, guild_id: GuildId, main_voice_id: ChannelId, red_team_voice_id: ChannelId, blue_team_voice_id: ChannelId) {
        let lobby = models::lobby::ActiveModel {
            guild_id: Set(guild_id.0 as i64),
            main_voice_id: Set(main_voice_id.0 as i64),
            red_team_voice_id: Set(red_team_voice_id.0 as i64),
            blue_team_voice_id: Set(blue_team_voice_id.0 as i64),
            ..Default::default()
        };

        lobby.insert(&self.connection)
            .await
            .expect("Could not insert lobby into database");
    }

    pub async fn get_lobby_by_channel(&self, guild_id: GuildId, channel_id: ChannelId) -> Option<models::lobby::Model> {
        models::lobby::Entity::find()
            .filter(models::lobby::Column::GuildId.eq(guild_id.0).and(
                models::lobby::Column::MainVoiceId.eq(channel_id.0 as i64)
                    .or(models::lobby::Column::RedTeamVoiceId.eq(channel_id.0 as i64))
                    .or(models::lobby::Column::BlueTeamVoiceId.eq(channel_id.0 as i64))
            ))
            .one(&self.connection)
            .await
            .expect("Could not get lobby from database")
    }
}

pub struct DatabaseContainer;

impl TypeMapKey for DatabaseContainer {
    type Value = Arc<RwLock<MixerDatabase>>;
}