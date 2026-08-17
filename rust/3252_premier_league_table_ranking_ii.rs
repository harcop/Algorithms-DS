/// LeetCode #3252 - Premier League Table Ranking II (SQL; Rust analogue)
/// team_stats: (team_id, team_name, matches_played, wins, draws, losses)
fn premier_league_table_ranking_ii(
    team_stats: Vec<(i32, String, i32, i32, i32, i32)>,
) -> Vec<(String, i32, i32, String)> {
    let total = team_stats.len() as i32;
    let mut teams: Vec<(String, i32)> = team_stats
        .into_iter()
        .map(|(_, name, _, wins, draws, _)| (name, wins * 3 + draws))
        .collect();
    teams.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let mut ranked: Vec<(String, i32, i32, String)> = Vec::new();
    for (i, (name, points)) in teams.into_iter().enumerate() {
        let position = if i > 0 && ranked[i - 1].1 == points {
            ranked[i - 1].2
        } else {
            (i + 1) as i32
        };
        let t1 = (total + 2) / 3;
        let t2 = (2 * total + 2) / 3;
        let tier = if position <= t1 {
            "Tier 1"
        } else if position <= t2 {
            "Tier 2"
        } else {
            "Tier 3"
        };
        ranked.push((name, points, position, tier.into()));
    }
    ranked
}

fn main() {
    let teams = vec![
        (1, "Chelsea".into(), 22, 13, 2, 7),
        (8, "Sheffield United".into(), 20, 18, 2, 0),
    ];
    println!("{:?}", premier_league_table_ranking_ii(teams));
}

#[cfg(test)]
mod tests {
    use super::premier_league_table_ranking_ii;

    #[test]
    fn example() {
        let teams = vec![
            (1, "Chelsea".into(), 22, 13, 2, 7),
            (2, "Nottingham Forest".into(), 27, 6, 6, 15),
            (3, "Liverpool".into(), 17, 1, 8, 8),
            (4, "Aston Villa".into(), 20, 1, 6, 13),
            (5, "Fulham".into(), 31, 18, 1, 12),
            (6, "Burnley".into(), 26, 6, 9, 11),
            (7, "Newcastle United".into(), 33, 11, 10, 12),
            (8, "Sheffield United".into(), 20, 18, 2, 0),
            (9, "Luton Town".into(), 5, 4, 0, 1),
            (10, "Everton".into(), 14, 2, 6, 6),
        ];
        assert_eq!(
            premier_league_table_ranking_ii(teams),
            vec![
                ("Sheffield United".into(), 56, 1, "Tier 1".into()),
                ("Fulham".into(), 55, 2, "Tier 1".into()),
                ("Newcastle United".into(), 43, 3, "Tier 1".into()),
                ("Chelsea".into(), 41, 4, "Tier 1".into()),
                ("Burnley".into(), 27, 5, "Tier 2".into()),
                ("Nottingham Forest".into(), 24, 6, "Tier 2".into()),
                ("Everton".into(), 12, 7, "Tier 2".into()),
                ("Luton Town".into(), 12, 7, "Tier 2".into()),
                ("Liverpool".into(), 11, 9, "Tier 3".into()),
                ("Aston Villa".into(), 9, 10, "Tier 3".into()),
            ]
        );
    }
}
