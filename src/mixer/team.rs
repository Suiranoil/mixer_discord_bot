use sea_orm::Iterable;
use std::collections::HashMap;

use crate::database::models::role::Role;
use crate::mixer::player::Player;
use crate::mixer::rating::Rating;

#[derive(Debug, Clone)]
pub struct Team {
    pub players: HashMap<(Role, i32), Option<usize>>,

    max_role: HashMap<Role, usize>,
    count_role: HashMap<Role, usize>,
}

impl Team {
    pub fn new(slots: Vec<Role>) -> Self {
        Self {
            players: {
                let mut players = HashMap::new();
                for role in Role::iter() {
                    for i in 0..slots.iter().filter(|slot| **slot == role).count() {
                        players.insert((role, i as i32), None);
                    }
                }
                players
            },
            max_role: {
                let mut max_role = HashMap::new();
                for role in Role::iter() {
                    max_role.insert(role, slots.iter().filter(|slot| **slot == role).count());
                }
                max_role
            },
            count_role: {
                let mut count_role = HashMap::new();
                for role in Role::iter() {
                    count_role.insert(role, 0);
                }
                count_role
            },
        }
    }

    pub fn count(&self) -> usize {
        self.count_role.values().sum()
    }

    pub fn count_role(&self, role: &Role) -> usize {
        *self.count_role.get(role).unwrap()
    }

    pub fn full_rating(&self, players: &[Player]) -> Rating {
        self.players
            .iter()
            .map(|((role, _), index)| {
                if let Some(index) = index {
                    *players[*index].ranks.get(role).unwrap()
                } else {
                    Rating::zero()
                }
            })
            .sum()
    }

    pub fn average_rating(&self, players: &[Player]) -> Rating {
        if self.players.len() == 0 {
            return Rating::zero();
        }

        self.full_rating(players) / self.players.len() as f32
    }

    pub fn full_rating_role(&self, role: &Role, players: &[Player]) -> Rating {
        self.players
            .iter()
            .filter(|((r, _), _)| r == role)
            .map(|((_, _), index)| {
                if let Some(index) = index {
                    *players[*index].ranks.get(role).unwrap()
                } else {
                    Rating::zero()
                }
            })
            .sum()
    }

    pub fn average_rating_role(&self, role: &Role, players: &[Player]) -> Rating {
        let count = self.players.iter().filter(|((r, _), _)| r == role).count();
        if count == 0 {
            return Rating::zero();
        }

        self.full_rating_role(role, players) / count as f32
    }

    pub fn has_slot(&self, role: &Role) -> bool {
        self.count_role(role) < *self.max_role.get(role).unwrap()
    }

    pub fn add_player(&mut self, index: usize, role: &Role) {
        if !self.has_slot(role) {
            panic!("No slot for role {:?}", role);
        }

        let count = self.count_role(role);
        self.players.insert((*role, count as i32), Some(index));
        self.count_role.insert(*role, count + 1);
    }
}
