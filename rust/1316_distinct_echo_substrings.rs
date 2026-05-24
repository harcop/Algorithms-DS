/// LeetCode #1316 - Distinct Echo Substrings
use std::collections::HashSet;

fn distinct_echo_substrings(text: String) -> i32 {
    let b = text.as_bytes();
    let n = b.len();
    let mut set = HashSet::new();
    for len in 1..=n / 2 {
        for i in 0..=n - 2 * len {
            if &b[i..i + len] == &b[i + len..i + 2 * len] {
                set.insert(b[i..i + len].to_vec());
            }
        }
    }
    set.len() as i32
}

fn main() {
    println!("{}", distinct_echo_substrings("abcabcabc".to_string()));

}


#[cfg(test)]
mod tests {
    use super::distinct_echo_substrings;

    #[test]
    fn example_one() {
        assert_eq!(distinct_echo_substrings("abcabcabc".to_string()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(distinct_echo_substrings("leetcodeleetcode".to_string()), 2);
    }
}
