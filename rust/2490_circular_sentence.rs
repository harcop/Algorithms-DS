/// LeetCode #2490 - Circular Sentence
fn is_circular_sentence(sentence: String) -> bool {
    let bytes = sentence.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] == b' ' && bytes[i - 1] != bytes[i + 1] {
            return false;
        }
    }
    bytes[0] == bytes[bytes.len() - 1]
}

fn main() {
    println!(
        "{}",
        is_circular_sentence("leetcode exercises sound delightful".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::is_circular_sentence;

    #[test]
    fn example_one() {
        assert!(is_circular_sentence(
            "leetcode exercises sound delightful".to_string()
        ));
    }

    #[test]
    fn example_two() {
        assert!(is_circular_sentence("eetcode".to_string()));
    }

    #[test]
    fn example_three() {
        assert!(!is_circular_sentence("Leetcode is cool".to_string()));
    }
}
