/// LeetCode #1477 - Find Two Non Overlapping Sub Arrays Each With Target Sum
use std::collections::HashMap;
fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
    let n = arr.len();
    let mut pre = HashMap::new();
    pre.insert(0i32, 0usize);
    let mut sum = 0i32;
    let mut best = vec![usize::MAX; n + 1];
    let mut ans = i32::MAX;
    for i in 0..n {
        sum += arr[i];
        if let Some(&j) = pre.get(&(sum - target)) {
            let len = i + 1 - j;
            if best[j] != usize::MAX {
                ans = ans.min((best[j] + len) as i32);
            }
            best[i + 1] = best[i].min(len);
        } else {
            best[i + 1] = best[i];
        }
        pre.insert(sum, i + 1);
    }
    if ans == i32::MAX { -1 } else { ans }
}
fn main() { println!("{}", min_sum_of_lengths(vec![3,2,2,4,3], 3)); }
#[cfg(test)]
mod tests {
    use super::min_sum_of_lengths;
    #[test]
    fn example_one() { assert_eq!(min_sum_of_lengths(vec![3,2,2,4,3], 3), 2); }
    #[test]
    fn example_two() { assert_eq!(min_sum_of_lengths(vec![7,3,4,7], 7), 2); }
    #[test]
    fn example_three() { assert_eq!(min_sum_of_lengths(vec![4,3,2,1,4,3,2,4,4], 6), 5); }
}