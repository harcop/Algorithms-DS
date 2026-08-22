/// LeetCode #3365 - Rearrange K Substrings to Form Target String
use std::collections::HashMap;

fn is_possible_to_rearrange(s: String, t: String, k: i32) -> bool {
    let n = s.len();
    let m = n / k as usize;
    let mut cnt: HashMap<&str, i32> = HashMap::new();
    for i in (0..n).step_by(m) {
        *cnt.entry(&s[i..i + m]).or_insert(0) += 1;
        *cnt.entry(&t[i..i + m]).or_insert(0) -= 1;
    }
    cnt.values().all(|&v| v == 0)
}

fn main() {
    println!(
        "{}",
        is_possible_to_rearrange("abcd".into(), "cdab".into(), 2)
    );
}

#[cfg(test)]
mod tests {
    use super::is_possible_to_rearrange;

    #[test]
    fn example1() {
        assert!(is_possible_to_rearrange("abcd".into(), "cdab".into(), 2));
    }

    #[test]
    fn example2() {
        assert!(is_possible_to_rearrange("aabbcc".into(), "bbaacc".into(), 3));
    }

    #[test]
    fn example3() {
        assert!(!is_possible_to_rearrange("aabbcc".into(), "bbaacc".into(), 2));
    }
}
