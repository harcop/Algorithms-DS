/// LeetCode #1679 - Max Number Of K Sum Pairs
use std::collections::HashMap;

fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut cnt = HashMap::new();
    let mut ans = 0i32;
    for x in nums {
        let need = k - x;
        if let Some(c) = cnt.get_mut(&need) {
            if *c > 0 { *c -= 1; ans += 1; continue; }
        }
        *cnt.entry(x).or_insert(0) += 1;
    }
    ans
}
fn main() { println!("{}", max_operations(vec![1,2,3,4], 5)); }
#[cfg(test)]
mod tests {
    use super::max_operations;
    #[test]
    fn example_one() { assert_eq!(max_operations(vec![1,2,3,4], 5), 2); }
}