/// LeetCode #151 - Reverse Words in a String
fn reverse_words(s: String) -> String {
    s.split_whitespace()
        .rev()
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    println!("{}", reverse_words("the sky is blue".to_string()));
}

#[cfg(test)]
mod tests {
    use super::reverse_words;

    #[test]
    fn example_one() {
        assert_eq!(
            reverse_words("the sky is blue".to_string()),
            "blue is sky the"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(reverse_words("  hello world  ".to_string()), "world hello");
    }

    #[test]
    fn example_three() {
        assert_eq!(
            reverse_words("a good   example".to_string()),
            "example good a"
        );
    }
}
