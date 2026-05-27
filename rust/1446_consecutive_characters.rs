/// LeetCode #1446 - Consecutive Characters
fn max_power(s: String) -> i32 {
    let b = s.as_bytes();
    if b.is_empty() { return 0; }
    let mut best = 1;
    let mut cur = 1;
    for i in 1..b.len() {
        if b[i] == b[i - 1] { cur += 1; } else { cur = 1; }
        best = best.max(cur);
    }
    best
}
fn main() { println!("{}", max_power("leetcode".into())); }
#[cfg(test)]
mod tests {
    use super::max_power;
    #[test]
    fn example_one() { assert_eq!(max_power("leetcode".into()), 2); }
    #[test]
    fn example_two() { assert_eq!(max_power("abbcccddddeeeeedcba".into()), 5); }
}