/// LeetCode #1347 - Minimum Number Of Steps To Make Two Strings Anagram

fn min_steps(s: String, t: String) -> i32 {
    let mut cnt = [0i32; 26];
    for b in s.bytes() {
        cnt[(b - b'a') as usize] += 1;
    }
    for b in t.bytes() {
        cnt[(b - b'a') as usize] -= 1;
    }
    (0..26).map(|i| (cnt[i]).abs()).sum()
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
