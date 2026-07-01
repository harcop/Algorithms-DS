/// LeetCode #2186 - Minimum Number of Steps to Make Two Strings Anagram II
use std::collections::HashMap;

fn min_steps(s: String, t: String) -> i32 {
    let mut cnt = HashMap::new();
    for b in s.bytes() {
        *cnt.entry(b).or_insert(0i32) += 1;
    }
    for b in t.bytes() {
        *cnt.entry(b).or_insert(0i32) -= 1;
    }
    cnt.values().map(|v| v.abs()).sum()
}

fn main() {
    println!("{}", min_steps("leetcode".into(), "coats".into()));
}

#[cfg(test)]
mod tests {
    use super::min_steps;

    #[test]
    fn example_one() {
        assert_eq!(min_steps("leetcode".into(), "coats".into()), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_steps("night".into(), "thing".into()), 0);
    }
}
