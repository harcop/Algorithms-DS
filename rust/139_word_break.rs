use std::collections::HashSet;

/// LeetCode #139 - Word Break
fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let dict: HashSet<String> = word_dict.into_iter().collect();
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for i in 1..=n {
        for j in 0..i {
            if dp[j] && dict.contains(&s[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }
    dp[n]
}

fn main() {
    println!(
        "{}",
        word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::word_break;

    #[test]
    fn example_one() {
        assert!(word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()],
        ));
    }

    #[test]
    fn example_two() {
        assert!(word_break(
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()],
        ));
    }

    #[test]
    fn example_three() {
        assert!(!word_break(
            "catsandog".to_string(),
            vec![
                "cats".to_string(),
                "dog".to_string(),
                "sand".to_string(),
                "and".to_string(),
                "cat".to_string(),
            ],
        ));
    }
}
