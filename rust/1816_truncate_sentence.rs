/// LeetCode #1816 - Truncate Sentence
fn truncate_sentence(s: String, k: i32) -> String {
    s.split_whitespace().take(k as usize).collect::<Vec<_>>().join(" ")
}

fn main() {
    println!(
        "{}",
        truncate_sentence("Hello how are you Contestant".into(), 4)
    );
}

#[cfg(test)]
mod tests {
    use super::truncate_sentence;

    #[test]
    fn example_one() {
        assert_eq!(
            truncate_sentence("Hello how are you Contestant".into(), 4),
            "Hello how are you"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            truncate_sentence("What is the solution to this problem".into(), 4),
            "What is the solution"
        );
    }
}
