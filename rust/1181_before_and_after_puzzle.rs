/// LeetCode #1181 - Before and After Puzzle
use std::collections::HashSet;

fn before_and_after_puzzles(phrases: Vec<String>) -> Vec<String> {
    let mut ans = HashSet::new();
    for p in &phrases {
        for q in &phrases {
            if p == q {
                continue;
            }
            let pw: Vec<&str> = p.split(' ').collect();
            let qw: Vec<&str> = q.split(' ').collect();
            if pw.last() != qw.first() {
                continue;
            }
            let merged = if pw.len() == 1 {
                q.clone()
            } else {
                format!("{} {}", pw[..pw.len() - 1].join(" "), q)
            };
            ans.insert(merged);
        }
    }
    let mut out: Vec<String> = ans.into_iter().collect();
    out.sort();
    out
}

fn main() {
    println!(
        "{:?}",
        before_and_after_puzzles(vec![
            "writing code".into(),
            "code rocks".into(),
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::before_and_after_puzzles;

    #[test]
    fn example_one() {
        assert_eq!(
            before_and_after_puzzles(vec![
                "writing code".into(),
                "code rocks".into(),
            ]),
            vec!["writing code rocks".to_string()]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            before_and_after_puzzles(vec![
                "mission statement".into(),
                "a quick bite to eat".into(),
                "a chip off the old block".into(),
                "chocolate bar".into(),
                "mission impossible".into(),
                "a man on a mission".into(),
                "block party".into(),
                "eat my words".into(),
                "bar of soap".into(),
            ]),
            vec![
                "a chip off the old block party".to_string(),
                "a man on a mission impossible".to_string(),
                "a man on a mission statement".to_string(),
                "a quick bite to eat my words".to_string(),
                "chocolate bar of soap".to_string(),
            ]
        );
    }
}
