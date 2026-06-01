/// LeetCode #1708 - Largest Subarray Length K
fn largest_subarray(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut best = 0usize;
    for i in 0..=nums.len() - k {
        if nums[i..i + k] > nums[best..best + k] {
            best = i;
        }
    }
    nums[best..best + k].to_vec()
}
fn main() { println!("{:?}", largest_subarray(vec![1,4,5,2,3], 3)); }
#[cfg(test)]
mod tests {
    use super::largest_subarray;
    #[test]
    fn example_one() { assert_eq!(largest_subarray(vec![1,4,5,2,3], 3), vec![5,2,3]); }
}