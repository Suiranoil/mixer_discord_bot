pub mod queries;

use sea_orm::{DatabaseConnection, SqlxPostgresConnector};
use serenity::prelude::TypeMapKey;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct MixerDatabase {
    connection: DatabaseConnection,
}

impl MixerDatabase {
    pub fn new(pool: PgPool) -> Self {
        Self {
            connection: SqlxPostgresConnector::from_sqlx_postgres_pool(pool),
        }
    }

    pub fn connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}

pub struct DatabaseContainer;

impl TypeMapKey for DatabaseContainer {
    type Value = Arc<RwLock<MixerDatabase>>;
}
