/// LeetCode #1749 - Maximum Absolute Sum of Any Subarray
fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let mut best = 0i32;
    let mut cur_max = 0i32;
    let mut cur_min = 0i32;
    for x in nums {
        cur_max = (cur_max + x).max(x);
        cur_min = (cur_min + x).min(x);
        best = best.max(cur_max).max(-cur_min);
    }
    best
}
fn main() { println!("{}", max_absolute_sum(vec![1, -3, 2, 3, -1])); }
#[cfg(test)]
mod tests {
    use super::max_absolute_sum;
    #[test]
    fn example_one() { assert_eq!(max_absolute_sum(vec![1, -3, 2, 3, -1]), 5); }
    #[test]
    fn example_two() { assert_eq!(max_absolute_sum(vec![2, -5, 1, -4, 3, -2]), 8); }
}
