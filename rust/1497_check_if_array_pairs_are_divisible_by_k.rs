/// LeetCode #1497 - Check If Array Pairs Are Divisible By K
use std::collections::HashMap;
fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
    if arr.len() % 2 != 0 { return false; }
    let mut cnt = HashMap::new();
    for x in arr {
        let r = ((x % k) + k) % k;
        *cnt.entry(r).or_insert(0) += 1;
    }
    for r in 0..k {
        let c = cnt.get(&r).copied().unwrap_or(0);
        if r == 0 {
            if c % 2 != 0 { return false; }
        } else if r * 2 < k {
            if c != cnt.get(&(k - r)).copied().unwrap_or(0) { return false; }
        }
    }
    true
}
fn main() { println!("{}", can_arrange(vec![1,-1,1,-1], 2)); }
#[cfg(test)]
mod tests {
    use super::can_arrange;
    #[test]
    fn example_one() { assert!(can_arrange(vec![1,-1,1,-1], 2)); }
    #[test]
    fn example_two() { assert!(can_arrange(vec![1,2,3,4,5,10,6,7,8,9], 5)); }
    #[test]
    fn example_three() { assert!(can_arrange(vec![1,2,3,4,5,9,6,7,8,10], 5)); }
}