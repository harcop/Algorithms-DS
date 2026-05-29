/// LeetCode #1525 - Number Of Good Ways To Split A String
fn num_splits(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut right = [0i32; 26];
    for &c in s { right[(c - b'a') as usize] += 1; }
    let mut right_distinct = right.iter().filter(|&&x| x > 0).count() as i32;
    let mut left = [0i32; 26];
    let mut left_distinct = 0;
    let mut ans = 0;
    for i in 0..n - 1 {
        let k = (s[i] - b'a') as usize;
        left[k] += 1;
        if left[k] == 1 { left_distinct += 1; }
        right[k] -= 1;
        if right[k] == 0 { right_distinct -= 1; }
        if left_distinct == right_distinct { ans += 1; }
    }
    ans
}
fn main() { println!("{}", num_splits("aacaba".into())); }
#[cfg(test)]
mod tests {
    use super::num_splits;
    #[test]
    fn example_one() { assert_eq!(num_splits("aacaba".into()), 2); }
    #[test]
    fn example_two() { assert_eq!(num_splits("abcd".into()), 1); }
}
