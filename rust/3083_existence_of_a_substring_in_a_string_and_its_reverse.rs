/// LeetCode #3083 - Existence of a Substring in a String and Its Reverse
use std::collections::HashSet;

fn is_substring_present(s: String) -> bool {
    let bytes = s.as_bytes();
    let mut pairs = HashSet::new();
    for w in bytes.windows(2) {
        pairs.insert((w[0], w[1]));
    }
    let rev: Vec<u8> = bytes.iter().copied().rev().collect();
    rev.windows(2).any(|w| pairs.contains(&(w[0], w[1])))
}

fn main() {
    println!("{}", is_substring_present("leetcode".into()));
}

#[cfg(test)]
mod tests {
    use super::is_substring_present;

    #[test]
    fn example1() {
        assert!(is_substring_present("leetcode".into()));
    }

    #[test]
    fn example2() {
        assert!(is_substring_present("abcba".into()));
    }

    #[test]
    fn example3() {
        assert!(!is_substring_present("abcd".into()));
    }
}
