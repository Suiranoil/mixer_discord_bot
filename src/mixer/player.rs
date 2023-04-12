use std::collections::HashMap;
use serenity::model::id::UserId;
use crate::database::models::player::Model;
use crate::mixer::rating::Rating;
use crate::mixer::role::Role;

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub(crate) id: i32,
    pub(crate) discord_id: UserId,
    pub(crate) bn_name: String,
    pub(crate) bn_tag: String,

    pub(crate) ranks: HashMap<Role, Rating>,
    pub(crate) flex: bool,
    pub(crate) priority_roles: Vec<Option<Role>>,
}


impl Player {
    pub fn new(model: Model) -> Self {
        Self {
            id: model.id,
            discord_id: UserId::from(model.discord_id as u64),
            bn_name: model.bn_name,
            bn_tag: model.bn_tag,

            ranks: vec![
                (Role::Tank, Rating::new(model.tank_rating, model.tank_rd, model.tank_volatility)),
                (Role::Dps, Rating::new(model.dps_rating, model.dps_rd, model.dps_volatility)),
                (Role::Support, Rating::new(model.support_rating, model.support_rd, model.support_volatility))
            ].into_iter().collect(),

            flex: model.flex,
            priority_roles: vec![
                model.primary_role,
                model.secondary_role, model.tertiary_role
            ].into_iter().map(|role| {
                match role {
                    -1 => None,
                    _ => Some(Role::from(role))
                }
            }).collect()
        }
    }

    pub fn to_model(self) -> Model {
        Model {
            id: self.id,
            discord_id: self.discord_id.0 as i64,
            bn_name: self.bn_name,
            bn_tag: self.bn_tag,
            tank_rating: self.ranks.get(&Role::Tank).unwrap().value,
            tank_rd: self.ranks.get(&Role::Tank).unwrap().rd,
            tank_volatility: self.ranks.get(&Role::Tank).unwrap().volatility,
            dps_rating: self.ranks.get(&Role::Dps).unwrap().value,
            dps_rd: self.ranks.get(&Role::Dps).unwrap().rd,
            dps_volatility: self.ranks.get(&Role::Dps).unwrap().volatility,
            support_rating: self.ranks.get(&Role::Support).unwrap().value,
            support_rd: self.ranks.get(&Role::Support).unwrap().rd,
            support_volatility: self.ranks.get(&Role::Support).unwrap().volatility,
            flex: self.flex,
            primary_role: Role::option_to_i32(self.priority_roles.get(0).unwrap().clone()),
            secondary_role: Role::option_to_i32(self.priority_roles.get(1).unwrap().clone()),
            tertiary_role: Role::option_to_i32(self.priority_roles.get(2).unwrap().clone()),
        }
    }

    pub fn base_priority(&self) -> HashMap<Role, f32> {
        let mut priorities = HashMap::new();
        let priority_points = 100.0;

        if self.flex {
            for role in Role::iter() {
                priorities.insert(role, 1.5 * (priority_points / (Role::iter().count()) as f32));
            }

            return priorities;
        }

        for (i, role) in self.priority_roles.iter().enumerate() {
            if let Some(role) = role {
                priorities.insert(role.clone(), priority_points / (i + 1) as f32);
            }
        }

        priorities
    }
}