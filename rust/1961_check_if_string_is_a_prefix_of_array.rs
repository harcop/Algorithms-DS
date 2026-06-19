/// LeetCode #1961 - Check If String Is a Prefix of Array
fn is_prefix_string(s: String, words: Vec<String>) -> bool {
    let n = s.len();
    let mut m = 0usize;
    for (i, w) in words.iter().enumerate() {
        m += w.len();
        if m == n {
            return words[..=i].concat() == s;
        }
        if m > n {
            return false;
        }
    }
    false
}

fn main() {
    println!(
        "{}",
        is_prefix_string(
            "iloveleetcode".into(),
            vec![
                "i".into(),
                "love".into(),
                "leetcode".into(),
                "apples".into(),
            ],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::is_prefix_string;

    #[test]
    fn example_one() {
        assert!(is_prefix_string(
            "iloveleetcode".into(),
            vec![
                "i".into(),
                "love".into(),
                "leetcode".into(),
                "apples".into(),
            ],
        ));
    }

    #[test]
    fn example_two() {
        assert!(!is_prefix_string(
            "iloveleetcode".into(),
            vec![
                "apples".into(),
                "i".into(),
                "love".into(),
                "leetcode".into(),
            ],
        ));
    }
}
