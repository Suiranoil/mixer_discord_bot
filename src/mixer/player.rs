use std::collections::HashMap;
use crate::database::models::player::Model;
use crate::mixer::role::Role;

#[derive(Debug, Clone)]
pub struct Player {
    pub(crate) id: i32,
    pub(crate) name: String,

    pub(crate) ranks: HashMap<Role, f32>,
    pub(crate) flex: bool,
    pub(crate) priority_roles: Vec<Option<Role>>,
}


impl Player {
    pub fn new(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.bn_name,
            ranks: vec![(Role::Tank, model.tank), (Role::Dps, model.dps), (Role::Support, model.support)].into_iter().collect(),
            flex: model.flex,
            priority_roles: vec![model.primary_role, model.secondary_role, model.tertiary_role].into_iter().map(|role| {
                match role {
                    -1 => None,
                    _ => Some(Role::from(role))
                }
            }).collect()
        }
    }

    pub fn base_priority(&self) -> HashMap<Role, f32> {
        let mut priorities = HashMap::new();

        if self.flex {
            for role in Role::iter() {
                priorities.insert(role, (self.priority_roles.len() / 2) as f32);
            }

            return priorities;
        }

        let count = self.priority_roles.iter().filter(|role| role.is_some()).count() as f32;
        let denominator = count * (count + 1.0) * (2.0 * count + 1.0) / 6.0;
        let priority_points = 100.0;

        for (i, role) in self.priority_roles.iter().enumerate() {
            if let Some(role) = role {
                priorities.insert(role.clone(), priority_points * (count - i as f32)*(count - i as f32) / denominator as f32);
            }
        }

        priorities
    }
}