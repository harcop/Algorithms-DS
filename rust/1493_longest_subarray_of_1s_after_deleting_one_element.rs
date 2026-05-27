/// LeetCode #1493 - Longest Subarray Of 1s After Deleting One Element
fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut l = 0usize;
    let mut zeros = 0i32;
    let mut best = 0i32;
    for r in 0..nums.len() {
        if nums[r] == 0 { zeros += 1; }
        while zeros > 1 {
            if nums[l] == 0 { zeros -= 1; }
            l += 1;
        }
        best = best.max((r - l) as i32);
    }
    best
}
fn main() { println!("{}", longest_subarray(vec![1,1,0,1])); }
#[cfg(test)]
mod tests {
    use super::longest_subarray;
    #[test]
    fn example_one() { assert_eq!(longest_subarray(vec![1,1,0,1]), 3); }
    #[test]
    fn example_two() { assert_eq!(longest_subarray(vec![0,1,1,1,0,1,1,0,1]), 5); }
    #[test]
    fn example_three() { assert_eq!(longest_subarray(vec![1,1,1]), 2); }
}