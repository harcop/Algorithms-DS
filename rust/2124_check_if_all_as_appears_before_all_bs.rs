/// LeetCode #2124 - Check if All A's Appears Before All B's
fn check_string(s: String) -> bool {
    !s.as_bytes().windows(2).any(|w| w == b"ba")
}

fn main() {
    println!("{}", check_string("aaabbb".into()));
}

#[cfg(test)]
mod tests {
    use super::check_string;

    #[test]
    fn example_one() {
        assert!(check_string("aaabbb".into()));
    }

    #[test]
    fn example_two() {
        assert!(!check_string("abab".into()));
    }

    #[test]
    fn example_three() {
        assert!(check_string("bbb".into()));
    }
}
