/// LeetCode #3390 - Longest Team Pass Streak (SQL; Rust analogue)
/// teams: (player_id, team_name)
/// passes: (pass_from, time_stamp, pass_to)
fn longest_streak(
    teams: Vec<(i32, String)>,
    mut passes: Vec<(i32, String, i32)>,
) -> Vec<(String, i32)> {
    use std::collections::HashMap;
    let team_of: HashMap<i32, String> = teams.into_iter().collect();
    passes.sort_by(|a, b| a.1.cmp(&b.1));
    let mut best: HashMap<String, i32> = HashMap::new();
    let mut cur: HashMap<String, i32> = HashMap::new();
    for (from, _, to) in passes {
        let Some(tf) = team_of.get(&from) else {
            continue;
        };
        let Some(tt) = team_of.get(&to) else {
            continue;
        };
        if tf == tt {
            let len = cur.get(tf).copied().unwrap_or(0) + 1;
            cur.insert(tf.clone(), len);
            let e = best.entry(tf.clone()).or_insert(0);
            *e = (*e).max(len);
        } else {
            cur.insert(tf.clone(), 0);
        }
    }
    let mut ans: Vec<_> = best.into_iter().collect();
    ans.sort_by(|a, b| a.0.cmp(&b.0));
    ans
}

fn main() {
    let teams = vec![
        (1, "Arsenal".into()),
        (2, "Arsenal".into()),
        (3, "Arsenal".into()),
        (4, "Arsenal".into()),
        (5, "Chelsea".into()),
        (6, "Chelsea".into()),
        (7, "Chelsea".into()),
        (8, "Chelsea".into()),
    ];
    let passes = vec![(1, "00:05".into(), 2)];
    println!("{:?}", longest_streak(teams, passes));
}

#[cfg(test)]
mod tests {
    use super::longest_streak;

    #[test]
    fn example() {
        let teams = vec![
            (1, "Arsenal".into()),
            (2, "Arsenal".into()),
            (3, "Arsenal".into()),
            (4, "Arsenal".into()),
            (5, "Chelsea".into()),
            (6, "Chelsea".into()),
            (7, "Chelsea".into()),
            (8, "Chelsea".into()),
        ];
        let passes = vec![
            (1, "00:05".into(), 2),
            (2, "00:07".into(), 3),
            (3, "00:08".into(), 4),
            (4, "00:10".into(), 5),
            (6, "00:15".into(), 7),
            (7, "00:17".into(), 8),
            (8, "00:20".into(), 6),
            (6, "00:22".into(), 5),
            (1, "00:25".into(), 2),
            (2, "00:27".into(), 3),
        ];
        assert_eq!(
            longest_streak(teams, passes),
            vec![("Arsenal".into(), 3), ("Chelsea".into(), 4)]
        );
    }
}
