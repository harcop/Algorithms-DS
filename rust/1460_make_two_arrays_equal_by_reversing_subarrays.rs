/// LeetCode #1460 - Make Two Arrays Equal By Reversing Subarrays
fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
    let mut a = arr;
    let mut b = target;
    a.sort_unstable();
    b.sort_unstable();
    a == b
}
fn main() { println!("{}", can_be_equal(vec![1,2,3,4], vec![2,4,1,3])); }
#[cfg(test)]
mod tests {
    use super::can_be_equal;
    #[test]
    fn example_one() { assert!(can_be_equal(vec![1,2,3,4], vec![2,4,1,3])); }
    #[test]
    fn example_two() { assert!(can_be_equal(vec![7], vec![7])); }
    #[test]
    fn example_three() { assert!(can_be_equal(vec![3,7], vec![7,3])); }
}