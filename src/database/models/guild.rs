use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "guilds")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i32,
    #[sea_orm(unique)]
    pub guild_id: i64,
    #[sea_orm(default_value = false)]
    pub verified: bool,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
