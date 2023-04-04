use std::collections::HashMap;
use crate::mixer::role::Role;
use crate::database::models::player::{Model as Player, Model};
use crate::mixer::role;

pub struct Mixer {
    players: Vec<Player>,
}

#[derive(Debug, Clone)]
pub struct MixedPlayer {
    pub role: Role,
    pub player: Player,
}

impl Mixer {
    pub fn new(players: Vec<Player>) -> Self {
        Self {
            players
        }
    }

    pub fn select_teams(self) -> Option<(Vec<MixedPlayer>, Vec<MixedPlayer>)> {
        let mut players = self.players.clone();

        // TODO! pass in the original priorities
        // let original_priorities = players.iter().cloned().map(|player| (player.id, 1)).collect::<HashMap<i32, i32>>();

        let mut team1 = Vec::new();
        let mut team2 = Vec::new();

        'outer: for i in 0..10 {
            let priorities = self.get_players_priority(&players, &team1, &team2);
            // let priorities = Self::priorities_to_array(priorities, &players);

            if let Some(player) = self.get_highest_priority_player(&players, &priorities) {
                if Self::full_team_rank(&team1) > Self::full_team_rank(&team2) {
                    if Self::has_slot_for(&team2, player.role) {
                        team2.push(player.clone());
                    }
                    else if Self::has_slot_for(&team1, player.role) {
                        team1.push(player.clone());
                    }
                    else {
                        continue;
                    }
                }
                else {
                    if Self::has_slot_for(&team1, player.role) {
                        team1.push(player.clone());
                    }
                    else if Self::has_slot_for(&team2, player.role) {
                        team2.push(player.clone());
                    }
                    else {
                        continue;
                    }
                }

                players.retain(|p| p.id != player.player.id);
            }
            else {
                return None;
            }
        }

        Some((team1, team2))
    }

    fn get_players_priority(&self, players: &Vec<Player>, team1: &Vec<MixedPlayer>, team2: &Vec<MixedPlayer>) -> HashMap<(Role, i32), f32> {
        let mut priorities = HashMap::new();

        if Self::calculate_priorities(players, team1, team2, Role::Tank, &mut priorities) {
            return priorities;
        }

        if Self::calculate_priorities(players, team1, team2, Role::Dps, &mut priorities) {
            return priorities;
        }

        if Self::calculate_priorities(players, team1, team2, Role::Support, &mut priorities) {
            return priorities;
        }

        priorities
    }

    fn average_rank(team1: &Vec<MixedPlayer>, team2: &Vec<MixedPlayer>, role: Role) -> f32 {
        let selected_players = team1.iter().filter(|player| player.role == role)
            .chain(team2.iter().filter(|player| player.role == role)).clone().collect::<Vec<&MixedPlayer>>();

        if selected_players.len() <= 0 {
            return 0.0;
        }

        selected_players.iter().map(|player| {
            match player.role {
                Role::Tank => player.player.tank,
                Role::Dps => player.player.dps,
                Role::Support => player.player.support,
                _ => 0.0
            }
        }).sum::<f32>() / selected_players.len() as f32
    }

    fn full_team_rank(team: &Vec<MixedPlayer>) -> f32 {
        if team.len() <= 0 {
            return 0.0;
        }

        team.iter().map(|p| {
            match p.role {
                Role::Tank => p.player.tank,
                Role::Dps => p.player.dps,
                Role::Support => p.player.support,
                _ => 0.0
            }
        }).sum::<f32>()
    }

    fn has_slot_for(team: &Vec<MixedPlayer>, role: Role) -> bool {
        let team_role_count = team.iter().filter(|p| p.role == role).count();

        match role {
            Role::Tank => team_role_count < 1,
            Role::Dps | Role::Support => team_role_count < 2,
            _ => false
        }
    }

    fn calculate_priorities(players: &Vec<Model>, team1: &Vec<MixedPlayer>, team2: &Vec<MixedPlayer>, expected: Role, priorities: &mut HashMap<(Role, i32), f32>) -> bool {
        let group_coefficients = vec![1.0, 5.0, 7.5, 10.0, 6.0];

        let team1_roles = team1.iter().map(|player| player.role).collect::<Vec<Role>>();
        let team2_roles = team2.iter().map(|player| player.role).collect::<Vec<Role>>();

        let team1_role_count = team1_roles.iter().filter(|role| **role == expected).count();
        let team2_role_count = team2_roles.iter().filter(|role| **role == expected).count();

        let max_role_players = match expected {
            Role::Tank => 2,
            Role::Dps | Role::Support => 4,
            _ => 0
        };

        let prioritize_role = (team1_role_count + team2_role_count) < max_role_players;

        if !prioritize_role {
            return false;
        }


        let otp = players.iter().filter(|player|
            player.primary_role == expected && player.secondary_role == Role::None && player.tertiary_role == Role::None
        ).cloned().collect::<Vec<Player>>();

        let primary = players.iter().filter(|player|
            player.primary_role == expected && (player.secondary_role != Role::None || player.tertiary_role != Role::None)
        ).cloned().collect::<Vec<Player>>();

        let secondary = players.iter().filter(|player|
            player.secondary_role == expected
        ).cloned().collect::<Vec<Player>>();

        let flex  = players.iter().filter(|player|
            player.flex
        ).cloned().collect::<Vec<Player>>();

        let tertiary = players.iter().filter(|player|
            player.tertiary_role == expected
        ).cloned().collect::<Vec<Player>>();


        let average_role_rank = Self::average_rank(team1, team2, expected);

        for player in otp {
            let rank = match expected {
                Role::Tank => player.tank,
                Role::Dps => player.dps,
                Role::Support => player.support,
                _ => 0.0
            };
            let skill_difference = Self::calculate_rank_difference(average_role_rank, rank);

            priorities.insert((expected, player.id), rank / group_coefficients[0] / (skill_difference + 1.0));
        }

        for player in primary {
            let rank = match expected {
                Role::Tank => player.tank,
                Role::Dps => player.dps,
                Role::Support => player.support,
                _ => 0.0
            };
            let skill_difference = Self::calculate_rank_difference(average_role_rank, rank);

            priorities.insert((expected, player.id), rank / group_coefficients[1] / (skill_difference + 1.0));
        }

        for player in secondary {
            let rank = match expected {
                Role::Tank => player.tank,
                Role::Dps => player.dps,
                Role::Support => player.support,
                _ => 0.0
            };
            let skill_difference = Self::calculate_rank_difference(average_role_rank, rank);

            priorities.insert((expected, player.id), rank / group_coefficients[2] / (skill_difference + 1.0));
        }

        for player in tertiary {
            let rank = match expected {
                Role::Tank => player.tank,
                Role::Dps => player.dps,
                Role::Support => player.support,
                _ => 0.0
            };
            let skill_difference = Self::calculate_rank_difference(average_role_rank, rank);

            priorities.insert((expected, player.id), rank / group_coefficients[3] / (skill_difference + 1.0));
        }

        for player in flex {
            let rank = match expected {
                Role::Tank => player.tank,
                Role::Dps => player.dps,
                Role::Support => player.support,
                _ => 0.0
            };
            let skill_difference = Self::calculate_rank_difference(average_role_rank, rank);

            priorities.insert((expected, player.id), rank / group_coefficients[4] / (skill_difference + 1.0));
        }

        true
    }

    fn calculate_rank_difference(average_rank: f32, rank: f32) -> f32 {
        match average_rank {
            0.0 => 0.0,
            _ => (rank - average_rank).abs()
        }
    }

    fn get_highest_priority_player(&self, players: &Vec<Player>, priorities: &HashMap<(Role, i32), f32>) -> Option<MixedPlayer> {
        let mut highest_priority = 0.0;
        let mut highest_priority_player = None;

        for role in vec![Role::Tank, Role::Dps, Role::Support] {
            for player in players {
                if let Some(priority) = priorities.get(&(role, player.id)) {
                    if *priority > highest_priority {
                        highest_priority = *priority;
                        highest_priority_player = Some(MixedPlayer {
                            role,
                            player: player.clone(),
                        });
                    }
                }
            }
        }

        highest_priority_player
    }
    fn priorities_to_array(priorities: HashMap<(Role, i32), f32>, players: &Vec<Player>) -> Vec<MixedPlayer> {
        let mut sorted: Vec<MixedPlayer> = vec![];
        for role in vec![Role::Tank, Role::Dps, Role::Support] {
            for player in players {
                if let Some(priority) = priorities.get(&(role, player.id)) {
                    sorted.push(MixedPlayer {
                        role,
                        player: player.clone()
                    })
                }
            }
        }

        sorted.sort_by(|p1, p2| priorities.get(&(p2.role, p2.player.id)).unwrap()
            .total_cmp(priorities.get(&(p1.role, p1.player.id)).unwrap())
        );

        sorted
    }
}