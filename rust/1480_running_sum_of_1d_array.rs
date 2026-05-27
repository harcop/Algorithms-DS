/// LeetCode #1480 - Running Sum Of 1d Array
fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut res = nums.clone();
    for i in 1..res.len() { res[i] += res[i - 1]; }
    res
}
fn main() { println!("{:?}", running_sum(vec![1,2,3,4])); }
#[cfg(test)]
mod tests {
    use super::running_sum;
    #[test]
    fn example_one() { assert_eq!(running_sum(vec![1,2,3,4]), vec![1,3,6,10]); }
    #[test]
    fn example_two() { assert_eq!(running_sum(vec![1,1,1,1,1]), vec![1,2,3,4,5]); }
}