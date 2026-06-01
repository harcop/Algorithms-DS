/// LeetCode #1636 - Sort Array By Increasing Frequency
fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut cnt = [0i32; 2001];
    for &x in &nums { cnt[(x + 1000) as usize] += 1; }
    let mut v = nums;
    v.sort_by_key(|&x| (cnt[(x + 1000) as usize], -x));
    v
}
fn main() { println!("{:?}", frequency_sort(vec![1,1,2,2,2,3])); }
#[cfg(test)]
mod tests {
    use super::frequency_sort;
    #[test]
    fn example_one() { assert_eq!(frequency_sort(vec![1,1,2,2,2,3]), vec![3,1,1,2,2,2]); }
}