/// LeetCode #1674 - Minimum Moves To Make Subarray Sum Equal
fn min_moves(nums: Vec<i32>) -> i32 {
    let mut v: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
    v.sort_unstable();
    let med = v[v.len() / 2];
    v.iter().map(|&x| (x - med).abs()).sum::<i64>() as i32
}
fn main() { println!("{}", min_moves(vec![1,2,3])); }
#[cfg(test)]
mod tests {
    use super::min_moves;
    #[test]
    fn example_one() { assert_eq!(min_moves(vec![1,2,3]), 2); }
}