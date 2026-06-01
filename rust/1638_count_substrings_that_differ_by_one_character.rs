/// LeetCode #1638 - Count Substrings That Differ By One Character
fn count_substrings(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut ans = 0i32;
    for i in 0..s.len() {
        for j in 0..t.len() {
            let mut diff = 0i32;
            for k in 0..s.len().min(t.len()).min(s.len() - i).min(t.len() - j) {
                if s[i + k] != t[j + k] {
                    diff += 1;
                    if diff > 1 { break; }
                }
                if diff == 1 { ans += 1; }
            }
        }
    }
    ans
}
fn main() { println!("{}", count_substrings("aba".into(), "baba".into())); }
#[cfg(test)]
mod tests {
    use super::count_substrings;
    #[test]
    fn example_one() { assert_eq!(count_substrings("aba".into(), "baba".into()), 6); }
    #[test]
    fn example_two() { assert_eq!(count_substrings("ab".into(), "bb".into()), 3); }
}