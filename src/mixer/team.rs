use std::collections::HashMap;
use crate::mixer::player::Player;
use crate::mixer::role::Role;

#[derive(Debug, Clone)]
pub struct Team {
    pub players: HashMap<(Role, i32), Option<Player>>
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
            }
        }
    }

    pub fn count(&self) -> usize {
        self.players.iter().filter(|(_, player)| player.is_some()).count()
    }

    pub fn count_role(&self, role: &Role) -> usize {
        self.players.iter().filter(|((r, _), player)| r == role && player.is_some()).count()
    }

    pub fn full_rank(&self) -> f32 {
        self.players.iter().map(|((role, _), player)| {
            if let Some(player) = player {
                player.ranks.get(role).unwrap().clone()
            } else {
                0.0
            }
        }).sum::<f32>()
    }

    pub fn average_rank(&self) -> f32 {
        if self.players.len() == 0 {
            return 0.0;
        }

        self.full_rank() / self.players.len() as f32
    }

    pub fn full_rank_role(&self, role: &Role) -> f32 {
        self.players.iter().filter(|((r, _), _)| r == role).map(|((_, _), player)| {
            if let Some(player) = player {
                player.ranks.get(&role).unwrap().clone()
            } else {
                0.0
            }
        }).sum::<f32>()
    }

    pub fn average_rank_role(&self, role: &Role) -> f32 {
        let count = self.players.iter().filter(|((r, _), _)| r == role).count();
        if count == 0 {
            return 0.0;
        }

        self.full_rank_role(role) / count as f32
    }

    pub fn has_slot(&self, role: &Role) -> bool {
        self.players.iter().filter(|((r, _), _)| r == role).any(|(_, player)| player.is_none())
    }

    pub fn add_player(&mut self, player: &Player, role: &Role) {
        let slot = self.players.iter().filter(|((r, _), _)| r == role).find(|(_, player)| player.is_none()).unwrap().0.clone();
        self.players.insert(slot, Some(player.clone()));
    }
}