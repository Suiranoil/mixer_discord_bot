use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "players")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub discord_id: i64,
    #[sea_orm(column_type = "Text")]
    pub bn_name: String,
    #[sea_orm(column_type = "Text")]
    pub bn_tag: String,
    #[sea_orm(default_value = 2500.0)]
    pub tank: f32,
    #[sea_orm(default_value = 2500.0)]
    pub dps: f32,
    #[sea_orm(default_value = 2500.0)]
    pub support: f32,

    #[sea_orm(default_value = true)]
    pub flex: bool,
    #[sea_orm(default_value = -1)]
    pub primary_role: i32,
    #[sea_orm(default_value = -1)]
    pub secondary_role: i32,
    #[sea_orm(default_value = -1)]
    pub tertiary_role: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}