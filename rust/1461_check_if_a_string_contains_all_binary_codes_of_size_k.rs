/// LeetCode #1461 - Check If A String Contains All Binary Codes Of Size K
use std::collections::HashSet;
fn has_all_codes(s: String, k: i32) -> bool {
    let k = k as usize;
    if s.len() < k { return false; }
    let b = s.as_bytes();
    let mut set = HashSet::new();
    let mut val = 0u32;
    for i in 0..b.len() {
        val = ((val << 1) | (b[i] - b'0') as u32) & ((1u32 << k) - 1);
        if i + 1 >= k { set.insert(val); }
    }
    set.len() == 1 << k
}
fn main() { println!("{}", has_all_codes("00110110".into(), 2)); }
#[cfg(test)]
mod tests {
    use super::has_all_codes;
    #[test]
    fn example_one() { assert!(has_all_codes("00110110".into(), 2)); }
    #[test]
    fn example_two() { assert!(has_all_codes("0110".into(), 1)); }
    #[test]
    fn example_three() { assert!(!has_all_codes("0110".into(), 2)); }
}