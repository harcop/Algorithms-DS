/// LeetCode #3246 - Premier League Table Ranking (SQL; Rust analogue)
/// team_stats: (team_id, team_name, matches_played, wins, draws, losses)
fn premier_league_table_ranking(
    team_stats: Vec<(i32, String, i32, i32, i32, i32)>,
) -> Vec<(i32, String, i32, i32)> {
    let mut teams: Vec<(i32, String, i32)> = team_stats
        .into_iter()
        .map(|(id, name, _, wins, draws, _)| (id, name, wins * 3 + draws))
        .collect();
    teams.sort_by(|a, b| b.2.cmp(&a.2).then(a.1.cmp(&b.1)));
    let mut ranked: Vec<(i32, String, i32, i32)> = Vec::new();
    for (i, (id, name, points)) in teams.into_iter().enumerate() {
        let position = if i > 0 && ranked[i - 1].2 == points {
            ranked[i - 1].3
        } else {
            (i + 1) as i32
        };
        ranked.push((id, name, points, position));
    }
    ranked
}

fn main() {
    let teams = vec![
        (1, "Manchester City".into(), 10, 6, 2, 2),
        (2, "Liverpool".into(), 10, 6, 2, 2),
    ];
    println!("{:?}", premier_league_table_ranking(teams));
}

#[cfg(test)]
mod tests {
    use super::premier_league_table_ranking;

    #[test]
    fn example() {
        let teams = vec![
            (1, "Manchester City".into(), 10, 6, 2, 2),
            (2, "Liverpool".into(), 10, 6, 2, 2),
            (3, "Chelsea".into(), 10, 5, 3, 2),
            (4, "Arsenal".into(), 10, 4, 4, 2),
            (5, "Tottenham".into(), 10, 3, 5, 2),
        ];
        assert_eq!(
            premier_league_table_ranking(teams),
            vec![
                (2, "Liverpool".into(), 20, 1),
                (1, "Manchester City".into(), 20, 1),
                (3, "Chelsea".into(), 18, 3),
                (4, "Arsenal".into(), 16, 4),
                (5, "Tottenham".into(), 14, 5),
            ]
        );
    }
}
