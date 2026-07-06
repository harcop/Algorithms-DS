/// LeetCode #2255 - Count Prefixes of a Given String
fn count_prefixes(words: Vec<String>, s: String) -> i32 {
    words.iter().filter(|word| s.starts_with(word.as_str())).count() as i32
}

fn main() {
    println!(
        "{}",
        count_prefixes(vec!["a".to_string(), "b".to_string(), "c".to_string()], "aaaa".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::count_prefixes;

    #[test]
    fn example_one() {
        assert_eq!(
            count_prefixes(vec!["a".to_string(), "b".to_string(), "c".to_string()], "aaaa".to_string()),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_prefixes(vec!["a".to_string(), "aa".to_string()], "aaa".to_string()),
            2
        );
    }
}
