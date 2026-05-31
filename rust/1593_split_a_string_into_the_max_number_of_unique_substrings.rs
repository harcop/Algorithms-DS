/// LeetCode #1593 - Split A String Into The Max Number Of Unique Substrings
fn max_unique_split(s: String) -> i32 {
    let b = s.as_bytes();
    let mut best = 0i32;
    fn dfs(start: usize, b: &[u8], seen: &mut std::collections::HashSet<Vec<u8>>, best: &mut i32) {
        if start == b.len() {
            *best = (*best).max(seen.len() as i32);
            return;
        }
        for end in start + 1..=b.len() {
            let sub = b[start..end].to_vec();
            if seen.insert(sub.clone()) {
                dfs(end, b, seen, best);
                seen.remove(&sub);
            }
        }
    }
    let mut seen = std::collections::HashSet::new();
    dfs(0, b, &mut seen, &mut best);
    best
}
fn main() { println!("{}", max_unique_split("ababccc".into())); }
#[cfg(test)]
mod tests {
    use super::max_unique_split;
    #[test]
    fn example_one() { assert_eq!(max_unique_split("ababccc".into()), 5); }
}