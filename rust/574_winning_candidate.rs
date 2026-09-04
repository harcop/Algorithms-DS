/// LeetCode #574 - Winning Candidate (SQL; Rust analogue)
use std::collections::HashMap;

fn winning_candidate(
    candidate: Vec<(i32, String)>,
    vote: Vec<(i32, i32)>,
) -> String {
    let names: HashMap<i32, String> = candidate.into_iter().collect();
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for (_, cid) in vote {
        *cnt.entry(cid).or_insert(0) += 1;
    }
    let winner = cnt.into_iter().max_by_key(|(id, c)| (*c, -id)).unwrap().0;
    names[&winner].clone()
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::winning_candidate;

    #[test]
    fn example() {
        let candidate = vec![
            (1, "A".into()),
            (2, "B".into()),
            (3, "C".into()),
            (4, "D".into()),
            (5, "E".into()),
        ];
        let vote = vec![(1, 2), (2, 4), (3, 3), (4, 2), (5, 5)];
        assert_eq!(winning_candidate(candidate, vote), "B");
    }
}
