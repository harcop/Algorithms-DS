/// LeetCode #3839 - Number of Prefix Connected Groups
use std::collections::HashMap;

fn prefix_connected(words: Vec<String>, k: i32) -> i32 {
    let k = k as usize;
    let mut cnt: HashMap<&str, i32> = HashMap::new();
    for w in &words {
        if w.len() >= k {
            *cnt.entry(&w[..k]).or_insert(0) += 1;
        }
    }
    cnt.values().filter(|&&v| v > 1).count() as i32
}

fn main() {
    println!(
        "{}",
        prefix_connected(
            vec![
                "apple".into(),
                "apply".into(),
                "banana".into(),
                "bandit".into()
            ],
            2
        )
    );
}

#[cfg(test)]
mod tests {
    use super::prefix_connected;

    #[test]
    fn example1() {
        assert_eq!(
            prefix_connected(
                vec![
                    "apple".into(),
                    "apply".into(),
                    "banana".into(),
                    "bandit".into()
                ],
                2
            ),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            prefix_connected(vec!["car".into(), "cat".into(), "cartoon".into()], 3),
            1
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            prefix_connected(
                vec![
                    "bat".into(),
                    "dog".into(),
                    "dog".into(),
                    "doggy".into(),
                    "bat".into()
                ],
                3
            ),
            2
        );
    }
}
