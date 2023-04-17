use sea_orm::entity::prelude::*;

use super::role::Role;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "players")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i32,

    #[sea_orm(unique)]
    pub discord_id: i64,
    pub bn_name: Option<String>,
    pub bn_tag: Option<String>,

    pub last_played: Option<DateTime>,

    #[sea_orm(default_value = 2500.0)]
    pub tank_rating: f32,
    #[sea_orm(default_value = 300.0)]
    pub tank_rd: f32,
    #[sea_orm(default_value = 0.06)]
    pub tank_volatility: f32,

    #[sea_orm(default_value = 2500.0)]
    pub dps_rating: f32,
    #[sea_orm(default_value = 300.0)]
    pub dps_rd: f32,
    #[sea_orm(default_value = 0.06)]
    pub dps_volatility: f32,

    #[sea_orm(default_value = 2500.0)]
    pub support_rating: f32,
    #[sea_orm(default_value = 300.0)]
    pub support_rd: f32,
    #[sea_orm(default_value = 0.06)]
    pub support_volatility: f32,

    #[sea_orm(default_value = true)]
    pub flex: bool,
    pub primary_role: Option<Role>,
    pub secondary_role: Option<Role>,
    pub tertiary_role: Option<Role>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
