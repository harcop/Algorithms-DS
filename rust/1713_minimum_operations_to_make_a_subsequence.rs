/// LeetCode #1713 - Minimum Operations To Make A Subsequence
fn min_operations(source: String, target: String) -> i32 {
    let s = source.as_bytes();
    let t = target.as_bytes();
    let mut pos = vec![vec![]; 26];
    for (i, &c) in s.iter().enumerate() {
        pos[(c - b'a') as usize].push(i);
    }
    let mut idx = 0usize;
    let mut matched = 0usize;
    for &c in t {
        let p = &pos[(c - b'a') as usize];
        match p.binary_search(&idx) {
            Ok(i) | Err(i) if i < p.len() => {
                idx = p[i] + 1;
                matched += 1;
            }
            _ => {}
        }
    }
    (t.len() - matched) as i32
}
fn main() { println!("{}", min_operations("abc".into(), "abcbc".into())); }
#[cfg(test)]
mod tests {
    use super::min_operations;
    #[test]
    fn example_one() { assert_eq!(min_operations("abc".into(), "abcbc".into()), 2); }
}