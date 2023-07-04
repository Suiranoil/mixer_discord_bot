pub use sea_orm_migration::prelude::*;

mod m20230704_104934_create_guilds_table;
mod m20230704_111535_create_lobbies_table;
mod m20230704_112326_create_role_type;
mod m20230704_113006_create_players_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230704_104934_create_guilds_table::Migration),
            Box::new(m20230704_111535_create_lobbies_table::Migration),
            Box::new(m20230704_112326_create_role_type::Migration),
            Box::new(m20230704_113006_create_players_table::Migration),
        ]
    }
}
