/// LeetCode #1647 - Minimum Deletions To Make Character Frequencies Unique
use std::collections::HashSet;

fn min_deletions(s: String) -> i32 {
    let mut cnt = [0i32; 26];
    for c in s.bytes() { cnt[(c - b'a') as usize] += 1; }
    let mut freqs: Vec<i32> = cnt.iter().copied().filter(|&x| x > 0).collect();
    freqs.sort_unstable_by(|a, b| b.cmp(a));
    let mut seen = HashSet::new();
    let mut del = 0i32;
    for f in freqs {
        let mut x = f;
        while x > 0 && seen.contains(&x) {
            x -= 1;
            del += 1;
        }
        seen.insert(x);
    }
    del
}
fn main() { println!("{}", min_deletions("aaabbbcc".into())); }
#[cfg(test)]
mod tests {
    use super::min_deletions;
    #[test]
    fn example_one() { assert_eq!(min_deletions("aaabbbcc".into()), 2); }
}