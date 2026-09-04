/// LeetCode #1212 - Team Scores in Football Tournament (SQL; Rust analogue)
use std::collections::HashMap;

fn team_scores(
    teams: Vec<(i32, String)>,
    matches: Vec<(i32, i32, i32, i32, i32)>,
) -> Vec<(i32, String, i32)> {
    let mut pts: HashMap<i32, i32> = teams.iter().map(|(id, _)| (*id, 0)).collect();
    for (_, host, guest, hg, gg) in matches {
        if hg > gg {
            *pts.entry(host).or_insert(0) += 3;
        } else if hg < gg {
            *pts.entry(guest).or_insert(0) += 3;
        } else {
            *pts.entry(host).or_insert(0) += 1;
            *pts.entry(guest).or_insert(0) += 1;
        }
    }
    let mut ans: Vec<(i32, String, i32)> = teams
        .into_iter()
        .map(|(id, name)| (id, name, *pts.get(&id).unwrap_or(&0)))
        .collect();
    ans.sort_by(|a, b| b.2.cmp(&a.2).then(a.0.cmp(&b.0)));
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::team_scores;

    #[test]
    fn example() {
        let teams = vec![
            (10, "Leetcode FC".into()),
            (20, "NewYork FC".into()),
            (30, "Atlanta FC".into()),
            (40, "Chicago FC".into()),
            (50, "Toronto FC".into()),
        ];
        let matches = vec![
            (1, 10, 20, 3, 0),
            (2, 30, 10, 2, 2),
            (3, 10, 50, 5, 1),
            (4, 20, 30, 1, 0),
            (5, 50, 30, 1, 0),
        ];
        assert_eq!(
            team_scores(teams, matches),
            vec![
                (10, "Leetcode FC".into(), 7),
                (20, "NewYork FC".into(), 3),
                (50, "Toronto FC".into(), 3),
                (30, "Atlanta FC".into(), 1),
                (40, "Chicago FC".into(), 0),
            ]
        );
    }
}
