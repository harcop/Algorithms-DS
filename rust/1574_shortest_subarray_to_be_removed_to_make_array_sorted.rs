/// LeetCode #1574 - Shortest Subarray To Be Removed To Make Array Sorted
fn find_length_of_shortest_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut r = n;
    while r > 0 && nums[r - 1] <= nums.get(r).copied().unwrap_or(i32::MAX) {
        r -= 1;
    }
    if r == 0 { return 0; }
    let mut ans = r as i32;
    let mut j = 0usize;
    for i in 0..n {
        if i > 0 && nums[i - 1] > nums[i] { break; }
        while j < r && nums[j] < nums[i] { j += 1; }
        ans = ans.min((j + n - r - i) as i32);
    }
    ans
}
fn main() { println!("{}", find_length_of_shortest_subarray(vec![1,2,3,10,4,2,3,5])); }
#[cfg(test)]
mod tests {
    use super::find_length_of_shortest_subarray;
    #[test]
    fn example_one() { assert_eq!(find_length_of_shortest_subarray(vec![1,2,3,10,4,2,3,5]), 3); }
    #[test]
    fn example_two() { assert_eq!(find_length_of_shortest_subarray(vec![5,4,3,2,1]), 1); }
}
