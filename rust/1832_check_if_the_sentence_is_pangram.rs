/// LeetCode #1832 - Check if the Sentence Is Pangram
fn check_if_pangram(sentence: String) -> bool {
    let mut seen = 0u32;
    for c in sentence.bytes() {
        seen |= 1 << (c - b'a');
    }
    seen == (1 << 26) - 1
}

fn main() {
    println!("{}", check_if_pangram("thequickbrownfoxjumpsoverthelazydog".to_string()));
}

#[cfg(test)]
mod tests {
    use super::check_if_pangram;

    #[test]
    fn example_one() {
        assert!(check_if_pangram(
            "thequickbrownfoxjumpsoverthelazydog".to_string()
        ));
    }

    #[test]
    fn example_two() {
        assert!(!check_if_pangram("leetcode".to_string()));
    }
}
