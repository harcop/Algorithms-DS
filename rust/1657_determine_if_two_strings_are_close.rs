/// LeetCode #1657 - Determine If Two Strings Are Close
fn close_strings(word1: String, word2: String) -> bool {
    let mut c1 = [0i32; 26];
    let mut c2 = [0i32; 26];
    for c in word1.bytes() { c1[(c - b'a') as usize] += 1; }
    for c in word2.bytes() { c2[(c - b'a') as usize] += 1; }
    let mut f1: Vec<i32> = c1.iter().copied().filter(|&x| x > 0).collect();
    let mut f2: Vec<i32> = c2.iter().copied().filter(|&x| x > 0).collect();
    f1.sort_unstable();
    f2.sort_unstable();
    f1 == f2 && c1.iter().zip(c2.iter()).all(|(&a, &b)| (a > 0) == (b > 0))
}
fn main() { println!("{}", close_strings("abc".into(), "bca".into())); }
#[cfg(test)]
mod tests {
    use super::close_strings;
    #[test]
    fn example_one() { assert!(close_strings("abc".into(), "bca".into())); }
    #[test]
    fn example_two() { assert!(!close_strings("a".into(), "aa".into())); }
}