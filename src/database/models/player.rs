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

    #[sea_orm(default_value = true)]
    pub flex: bool,

    #[sea_orm(default_value = 2500.0)]
    pub tank_rating: f32,
    #[sea_orm(default_value = 580.0)]
    pub tank_rd: f32,
    #[sea_orm(default_value = 0.06)]
    pub tank_volatility: f32,

    #[sea_orm(default_value = 2500.0)]
    pub dps_rating: f32,
    #[sea_orm(default_value = 580.0)]
    pub dps_rd: f32,
    #[sea_orm(default_value = 0.06)]
    pub dps_volatility: f32,

    #[sea_orm(default_value = 2500.0)]
    pub support_rating: f32,
    #[sea_orm(default_value = 580.0)]
    pub support_rd: f32,
    #[sea_orm(default_value = 0.06)]
    pub support_volatility: f32,

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