/// LeetCode #1752 - Check if Array Is Sorted and Rotated
fn check(nums: Vec<i32>) -> bool {
    let mut drops = 0usize;
    let n = nums.len();
    for i in 0..n {
        if nums[i] > nums[(i + 1) % n] {
            drops += 1;
            if drops > 1 {
                return false;
            }
        }
    }
    true
}
fn main() { println!("{}", check(vec![3, 4, 5, 1, 2])); }
#[cfg(test)]
mod tests {
    use super::check;
    #[test]
    fn example_one() { assert!(check(vec![3, 4, 5, 1, 2])); }
    #[test]
    fn example_two() { assert!(!check(vec![2, 1, 3, 4])); }
    #[test]
    fn example_three() { assert!(check(vec![3, 4, 5, 2, 3])); }
}
