/// LeetCode #1783 - Grand Slam Titles (SQL; Rust analogue)
use std::collections::HashMap;

fn grand_slam_titles(
    players: Vec<(i32, String)>,
    championships: Vec<(i32, i32, i32, i32, i32)>,
) -> Vec<(i32, String, i32)> {
    let names: HashMap<i32, String> = players.into_iter().collect();
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for (_, w, fr, us, au) in championships {
        for pid in [w, fr, us, au] {
            *counts.entry(pid).or_insert(0) += 1;
        }
    }
    let mut ans: Vec<(i32, String, i32)> = counts
        .into_iter()
        .filter(|(_, c)| *c > 0)
        .map(|(pid, c)| (pid, names.get(&pid).cloned().unwrap_or_default(), c))
        .collect();
    ans.sort_by_key(|t| t.0);
    ans
}

fn main() {
    let players = vec![
        (1, "Nadal".into()),
        (2, "Federer".into()),
        (3, "Novak".into()),
    ];
    let championships = vec![
        (2018, 1, 1, 1, 1),
        (2019, 1, 1, 2, 2),
        (2020, 2, 1, 2, 2),
    ];
    println!("{:?}", grand_slam_titles(players, championships));
}

#[cfg(test)]
mod tests {
    use super::grand_slam_titles;

    #[test]
    fn example_one() {
        let players = vec![
            (1, "Nadal".into()),
            (2, "Federer".into()),
            (3, "Novak".into()),
        ];
        let championships = vec![
            (2018, 1, 1, 1, 1),
            (2019, 1, 1, 2, 2),
            (2020, 2, 1, 2, 2),
        ];
        let mut got = grand_slam_titles(players, championships);
        got.sort_by_key(|t| t.0);
        assert_eq!(
            got,
            vec![
                (1, "Nadal".into(), 7),
                (2, "Federer".into(), 5),
            ]
        );
    }
}
