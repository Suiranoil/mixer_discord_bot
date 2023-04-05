use crate::mixer::role::Role;
use crate::mixer::player::Player;
use crate::mixer::team::Team;


pub fn mix_players(players: &Vec<Player>, slots: Vec<Role>) -> Option<(Team, Team)> {
    let mut players = players.clone();
    let mut team1 = Team::new(slots.clone());
    let mut team2 = Team::new(slots.clone());

    for _ in 0..(slots.len()*2) {
        let mut priorities = calculate_priorities(&players, &team1, &team2);
        priorities.sort_by(|(_, _, p1), (_, _, p2)| p2.partial_cmp(p1).unwrap());

        for (player, role, _) in priorities {
            if team1.full_rank() > team2.full_rank() {
                if team2.has_slot(&role) {
                    team2.add_player(&player, &role);
                    players.retain(|p| p.id != player.id);
                    break;
                }
            } else {
                if team1.has_slot(&role) {
                    team1.add_player(&player, &role);
                    players.retain(|p| p.id != player.id);
                    break;
                }
            }
        }
    }

    if team1.count() < slots.len() || team2.count() < slots.len() {
        return None;
    }

    Some((team1, team2))
}


fn calculate_priorities(players: &Vec<Player>, team1: &Team, team2: &Team) -> Vec<(Player, Role, f32)> {
    let mut priorities = Vec::new();

    for player in players {
        for (role, priority) in player.base_priority() {
            priorities.push((player.clone(), role, priority));
        }
    }

    for item in &mut priorities {
        let (player, role, _) = item;
        let empty_teams = team1.count_role(role) == 0 && team2.count_role(role) == 0;

        let role_diff_rank = 1.0 + {
            if empty_teams {
                0.0
            } else {
                (player.ranks[role] - (team1.full_rank_role(role) - team2.full_rank_role(role)).abs()).abs()
            }
        } as f32;

        let sum_average_rank = team1.average_rank_role(role) + team2.average_rank_role(role);
        let role_diff_avg_rank = 1.0 + {
            if empty_teams {
                let filtered_players = players.iter().filter(|player| player.priority_roles.contains(&Some(role.clone())) || player.flex).collect::<Vec<&Player>>();
                (player.ranks[role] - filtered_players.iter().map(|player| player.ranks[role]).sum::<f32>() / filtered_players.len() as f32).abs()
            } else {
                (player.ranks[role] - sum_average_rank).abs()
            }
        } as f32;

        let team_rank_difference = (team1.average_rank_role(role) - team2.average_rank_role(role)).abs();
        let rank_difference_weight = 1.0 + 1.5 * team_rank_difference;

        let complex_coefficient = role_diff_rank * role_diff_avg_rank * rank_difference_weight;
        item.2 *= player.ranks[role] / complex_coefficient;
    }

    priorities
}
