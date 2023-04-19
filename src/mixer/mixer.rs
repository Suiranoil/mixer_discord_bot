use itertools::Itertools;
use std::cmp::Ordering;

use crate::database::models::role::Role;
use crate::mixer::player::Player;
use crate::mixer::team::Team;

struct PlayerRoleEntry {
    pub index: usize,
    pub role: Role,
    pub priority: f32,
}

impl Eq for PlayerRoleEntry {}

impl PartialEq for PlayerRoleEntry {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

fn get_role_entries(
    entries: Vec<PlayerRoleEntry>,
) -> (
    Vec<PlayerRoleEntry>,
    Vec<PlayerRoleEntry>,
    Vec<PlayerRoleEntry>,
) {
    let (tanks, rest) = entries
        .into_iter()
        .partition::<Vec<_>, _>(|e| e.role == Role::Tank);
    let (dps, supports) = rest.into_iter().partition(|e| e.role == Role::DPS);

    (tanks, dps, supports)
}

fn get_combinations(entries: &[PlayerRoleEntry], count: usize) -> Vec<Vec<&PlayerRoleEntry>> {
    entries
        .into_iter()
        .combinations(count)
        .sorted_by(|a, b| {
            let a = a.iter().map(|e| e.priority).sum::<f32>();
            let b = b.iter().map(|e| e.priority).sum::<f32>();
            b.partial_cmp(&a).unwrap_or(Ordering::Equal)
        })
        .collect_vec()
}

pub fn mix_players(players: &[Player], slots: Vec<Role>) -> Option<(Team, Team)> {
    let players = players.to_vec();

    let entries = calculate_priorities(&players);
    let (tanks, dps, supports) = get_role_entries(entries);

    let tank_count = slots.iter().filter(|r| **r == Role::Tank).count();
    let support_count = slots.iter().filter(|r| **r == Role::Support).count();
    let dps_count = slots.iter().filter(|r| **r == Role::DPS).count();

    let tank_combos = get_combinations(&tanks, tank_count);
    let dps_combos = get_combinations(&dps, dps_count);
    let support_combos = get_combinations(&supports, support_count);

    let mut best_team1 = None;
    let mut best_team2 = None;
    let mut best_diff = None;

    let threshold = 300.0;

    // this is awful, but it works
    for tank1_combo in &tank_combos {
        for tank2_combo in &tank_combos {
            if tank1_combo
                .iter()
                .any(|e| tank2_combo.iter().any(|e2| e.index == e2.index))
            {
                continue;
            }

            for dps1_combo in &dps_combos {
                if tank1_combo
                    .iter()
                    .any(|e| dps1_combo.iter().any(|e2| e.index == e2.index))
                {
                    continue;
                }
                if tank2_combo
                    .iter()
                    .any(|e| dps1_combo.iter().any(|e2| e.index == e2.index))
                {
                    continue;
                }

                for dps2_combo in &dps_combos {
                    if tank1_combo
                        .iter()
                        .any(|e| dps2_combo.iter().any(|e2| e.index == e2.index))
                    {
                        continue;
                    }
                    if tank2_combo
                        .iter()
                        .any(|e| dps2_combo.iter().any(|e2| e.index == e2.index))
                    {
                        continue;
                    }
                    if dps1_combo
                        .iter()
                        .any(|e| dps2_combo.iter().any(|e2| e.index == e2.index))
                    {
                        continue;
                    }

                    for support1_combo in &support_combos {
                        if tank1_combo
                            .iter()
                            .any(|e| support1_combo.iter().any(|e2| e.index == e2.index))
                        {
                            continue;
                        }
                        if tank2_combo
                            .iter()
                            .any(|e| support1_combo.iter().any(|e2| e.index == e2.index))
                        {
                            continue;
                        }
                        if dps1_combo
                            .iter()
                            .any(|e| support1_combo.iter().any(|e2| e.index == e2.index))
                        {
                            continue;
                        }
                        if dps2_combo
                            .iter()
                            .any(|e| support1_combo.iter().any(|e2| e.index == e2.index))
                        {
                            continue;
                        }

                        for support2_combo in &support_combos {
                            if tank1_combo
                                .iter()
                                .any(|e| support2_combo.iter().any(|e2| e.index == e2.index))
                            {
                                continue;
                            }
                            if tank2_combo
                                .iter()
                                .any(|e| support2_combo.iter().any(|e2| e.index == e2.index))
                            {
                                continue;
                            }
                            if dps1_combo
                                .iter()
                                .any(|e| support2_combo.iter().any(|e2| e.index == e2.index))
                            {
                                continue;
                            }
                            if dps2_combo
                                .iter()
                                .any(|e| support2_combo.iter().any(|e2| e.index == e2.index))
                            {
                                continue;
                            }
                            if support1_combo
                                .iter()
                                .any(|e| support2_combo.iter().any(|e2| e.index == e2.index))
                            {
                                continue;
                            }

                            let mut team1 = Team::new(slots.clone());
                            let mut team2 = Team::new(slots.clone());

                            for entry in tank1_combo
                                .iter()
                                .chain(dps1_combo.iter())
                                .chain(support1_combo.iter())
                            {
                                team1.add_player(entry.index, &entry.role);
                            }

                            for entry in tank2_combo
                                .iter()
                                .chain(dps2_combo.iter())
                                .chain(support2_combo.iter())
                            {
                                team2.add_player(entry.index, &entry.role);
                            }

                            let diff_rating = (team1.full_rating(&players).value
                                - team2.full_rating(&players).value)
                                .abs();
                            let diff = diff_rating;

                            if diff + threshold < best_diff.unwrap_or(f32::MAX) {
                                if diff < threshold {
                                    return Some((team1, team2));
                                }

                                best_team1 = Some(team1);
                                best_team2 = Some(team2);
                                best_diff = Some(diff);
                            }
                        }
                    }
                }
            }
        }
    }

    if let (Some(team1), Some(team2)) = (best_team1, best_team2) {
        Some((team1, team2))
    } else {
        None
    }
}

fn calculate_priorities(players: &[Player]) -> Vec<PlayerRoleEntry> {
    let mut priorities = Vec::new();

    for (i, player) in players.iter().enumerate() {
        for (role, priority) in player.base_priority() {
            priorities.push(PlayerRoleEntry {
                index: i,
                role,
                priority,
            });
        }
    }

    priorities
}
