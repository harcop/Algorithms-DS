/// LeetCode #2820 - Election Results (SQL problem; Rust analogue)
use std::collections::HashMap;

fn election_winners(votes: Vec<(String, Option<String>)>) -> Vec<String> {
    let mut per_voter: HashMap<String, Vec<String>> = HashMap::new();
    for (voter, candidate) in votes {
        if let Some(c) = candidate {
            per_voter.entry(voter).or_default().push(c);
        }
    }

    let mut totals: HashMap<String, f64> = HashMap::new();
    for candidates in per_voter.values() {
        let share = 1.0 / candidates.len() as f64;
        for c in candidates {
            *totals.entry(c.clone()).or_default() += share;
        }
    }

    let max_vote = totals.values().copied().fold(0.0f64, f64::max);
    let mut winners: Vec<String> = totals
        .into_iter()
        .filter(|(_, v)| (*v - max_vote).abs() < 1e-9)
        .map(|(c, _)| c)
        .collect();
    winners.sort();
    winners
}

fn main() {
    let votes = vec![
        ("Kathy".into(), None),
        ("Charles".into(), Some("Ryan".into())),
        ("Charles".into(), Some("Christine".into())),
        ("Charles".into(), Some("Kathy".into())),
        ("Benjamin".into(), Some("Christine".into())),
        ("Anthony".into(), Some("Ryan".into())),
        ("Edward".into(), Some("Ryan".into())),
        ("Terry".into(), None),
        ("Evelyn".into(), Some("Kathy".into())),
        ("Arthur".into(), Some("Christine".into())),
    ];
    println!("{:?}", election_winners(votes));
}

#[cfg(test)]
mod tests {
    use super::election_winners;

    #[test]
    fn example_one() {
        let votes = vec![
            ("Kathy".into(), None),
            ("Charles".into(), Some("Ryan".into())),
            ("Charles".into(), Some("Christine".into())),
            ("Charles".into(), Some("Kathy".into())),
            ("Benjamin".into(), Some("Christine".into())),
            ("Anthony".into(), Some("Ryan".into())),
            ("Edward".into(), Some("Ryan".into())),
            ("Terry".into(), None),
            ("Evelyn".into(), Some("Kathy".into())),
            ("Arthur".into(), Some("Christine".into())),
        ];
        assert_eq!(
            election_winners(votes),
            vec!["Christine".to_string(), "Ryan".to_string()]
        );
    }
}
