/// LeetCode #1811 - Find Interview Candidates (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn interview_candidates(
    contests: Vec<(i32, i32, i32, i32)>,
    users: Vec<(i32, String, String)>,
) -> Vec<(String, String)> {
    let mut gold: HashMap<i32, i32> = HashMap::new();
    let mut medals: HashMap<i32, Vec<i32>> = HashMap::new();
    for (contest_id, g, s, b) in contests {
        *gold.entry(g).or_insert(0) += 1;
        medals.entry(g).or_default().push(contest_id);
        medals.entry(s).or_default().push(contest_id);
        medals.entry(b).or_default().push(contest_id);
    }
    let mut cand: HashSet<i32> = HashSet::new();
    for (uid, cnt) in &gold {
        if *cnt >= 3 {
            cand.insert(*uid);
        }
    }
    for (uid, mut ids) in medals {
        ids.sort();
        ids.dedup();
        let mut run = 1;
        for i in 1..ids.len() {
            if ids[i] == ids[i - 1] + 1 {
                run += 1;
                if run >= 3 {
                    cand.insert(uid);
                }
            } else {
                run = 1;
            }
        }
    }
    let mut ans: Vec<(String, String)> = users
        .into_iter()
        .filter(|(id, _, _)| cand.contains(id))
        .map(|(_, mail, name)| (name, mail))
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("{:?}", interview_candidates(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::interview_candidates;

    #[test]
    fn example_one() {
        let contests = vec![
            (190, 1, 5, 2),
            (191, 2, 3, 5),
            (192, 5, 2, 3),
            (193, 1, 3, 5),
            (194, 4, 5, 2),
            (195, 4, 2, 1),
            (196, 1, 5, 2),
        ];
        let users = vec![
            (1, "sarah@leetcode.com".into(), "Sarah".into()),
            (2, "bob@leetcode.com".into(), "Bob".into()),
            (3, "alice@leetcode.com".into(), "Alice".into()),
            (4, "hercy@leetcode.com".into(), "Hercy".into()),
            (5, "quarz@leetcode.com".into(), "Quarz".into()),
        ];
        assert_eq!(
            interview_candidates(contests, users),
            vec![
                ("Alice".into(), "alice@leetcode.com".into()),
                ("Bob".into(), "bob@leetcode.com".into()),
                ("Quarz".into(), "quarz@leetcode.com".into()),
                ("Sarah".into(), "sarah@leetcode.com".into()),
            ]
        );
    }
}
