/// LeetCode #3384 - Team Dominance by Pass Success (SQL; Rust analogue)
/// teams: (player_id, team_name)
/// passes: (pass_from, time_stamp, pass_to)
fn team_dominance(
    teams: Vec<(i32, String)>,
    passes: Vec<(i32, String, i32)>,
) -> Vec<(String, i32, i32)> {
    use std::collections::HashMap;
    let team_of: HashMap<i32, String> = teams.into_iter().collect();
    let mut scores: HashMap<(String, i32), i32> = HashMap::new();
    for (from, ts, to) in passes {
        let Some(tf) = team_of.get(&from) else {
            continue;
        };
        let Some(tt) = team_of.get(&to) else {
            continue;
        };
        let half = if ts.as_str() <= "45:00" { 1 } else { 2 };
        let delta = if tf == tt { 1 } else { -1 };
        *scores.entry((tf.clone(), half)).or_insert(0) += delta;
    }
    let mut ans: Vec<_> = scores
        .into_iter()
        .map(|((name, half), d)| (name, half, d))
        .collect();
    ans.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    ans
}

fn main() {
    let teams = vec![
        (1, "Arsenal".into()),
        (2, "Arsenal".into()),
        (3, "Arsenal".into()),
        (4, "Chelsea".into()),
        (5, "Chelsea".into()),
        (6, "Chelsea".into()),
    ];
    let passes = vec![(1, "00:15".into(), 2)];
    println!("{:?}", team_dominance(teams, passes));
}

#[cfg(test)]
mod tests {
    use super::team_dominance;

    #[test]
    fn example() {
        let teams = vec![
            (1, "Arsenal".into()),
            (2, "Arsenal".into()),
            (3, "Arsenal".into()),
            (4, "Chelsea".into()),
            (5, "Chelsea".into()),
            (6, "Chelsea".into()),
        ];
        let passes = vec![
            (1, "00:15".into(), 2),
            (2, "00:45".into(), 3),
            (3, "01:15".into(), 1),
            (4, "00:30".into(), 1),
            (2, "46:00".into(), 3),
            (3, "46:15".into(), 4),
            (1, "46:45".into(), 2),
            (5, "46:30".into(), 6),
        ];
        assert_eq!(
            team_dominance(teams, passes),
            vec![
                ("Arsenal".into(), 1, 3),
                ("Arsenal".into(), 2, 1),
                ("Chelsea".into(), 1, -1),
                ("Chelsea".into(), 2, 1),
            ]
        );
    }
}
