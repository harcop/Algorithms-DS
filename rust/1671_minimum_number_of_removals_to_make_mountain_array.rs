/// LeetCode #1671 - Minimum Number Of Removals To Make Mountain Array
fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut lis = vec![1i32; n];
    for i in 0..n {
        for j in 0..i {
            if nums[j] < nums[i] { lis[i] = lis[i].max(lis[j] + 1); }
        }
    }
    let mut lds = vec![1i32; n];
    for i in (0..n).rev() {
        for j in (i + 1..n).rev() {
            if nums[j] < nums[i] { lds[i] = lds[i].max(lds[j] + 1); }
        }
    }
    let mut best = 0i32;
    for i in 0..n {
        if lis[i] > 1 && lds[i] > 1 { best = best.max(lis[i] + lds[i] - 1); }
    }
    n as i32 - best
}
fn main() { println!("{}", minimum_mountain_removals(vec![1,3,1])); }
#[cfg(test)]
mod tests {
    use super::minimum_mountain_removals;
    #[test]
    fn example_one() { assert_eq!(minimum_mountain_removals(vec![1,3,1]), 0); }
    #[test]
    fn example_two() { assert_eq!(minimum_mountain_removals(vec![2,1,1,5,6,2,3,1]), 3); }
}