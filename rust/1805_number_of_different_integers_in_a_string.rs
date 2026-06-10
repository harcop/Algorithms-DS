/// LeetCode #1805 - Number of Different Integers in a String
use std::collections::HashSet;

fn num_different_integers(word: String) -> i32 {
    let b = word.as_bytes();
    let n = b.len();
    let mut seen = HashSet::new();
    let mut i = 0usize;
    while i < n {
        if b[i].is_ascii_digit() {
            while i < n && b[i] == b'0' {
                i += 1;
            }
            let start = i;
            while i < n && b[i].is_ascii_digit() {
                i += 1;
            }
            if start < i {
                seen.insert(&word[start..i]);
            }
        }
        i += 1;
    }
    seen.len() as i32
}

fn main() {
    println!("{}", num_different_integers("a123bc34d8ef34".into()));
}

#[cfg(test)]
mod tests {
    use super::num_different_integers;

    #[test]
    fn example_one() {
        assert_eq!(num_different_integers("a123bc34d8ef34".into()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_different_integers("leet1234code234".into()), 2);
    }
}
