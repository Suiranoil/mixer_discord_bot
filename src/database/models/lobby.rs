use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "lobbies")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub guild_id: i64,
    pub main_voice_id: i64,
    pub red_team_voice_id: i64,
    pub blue_team_voice_id: i64,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}
