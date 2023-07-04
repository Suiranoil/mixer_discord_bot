use super::sea_orm_active_enums::Role;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "players")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub discord_id: i64,
    pub bn_name: Option<String>,
    pub bn_tag: Option<String>,
    pub last_played: Option<DateTime>,
    #[sea_orm(column_type = "Float")]
    pub tank_rating: f32,
    #[sea_orm(column_type = "Float")]
    pub tank_rd: f32,
    #[sea_orm(column_type = "Float")]
    pub tank_volatility: f32,
    #[sea_orm(column_type = "Float")]
    pub dps_rating: f32,
    #[sea_orm(column_type = "Float")]
    pub dps_rd: f32,
    #[sea_orm(column_type = "Float")]
    pub dps_volatility: f32,
    #[sea_orm(column_type = "Float")]
    pub support_rating: f32,
    #[sea_orm(column_type = "Float")]
    pub support_rd: f32,
    #[sea_orm(column_type = "Float")]
    pub support_volatility: f32,
    pub flex: bool,
    pub primary_role: Option<Role>,
    pub secondary_role: Option<Role>,
    pub tertiary_role: Option<Role>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
