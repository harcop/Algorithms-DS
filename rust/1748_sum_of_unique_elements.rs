/// LeetCode #1748 - Sum of Unique Elements
fn sum_of_unique(nums: Vec<i32>) -> i32 {
    let mut cnt = [0i32; 101];
    for x in nums {
        cnt[x as usize] += 1;
    }
    (0..=100).filter(|&i| cnt[i] == 1).map(|i| i as i32).sum()
}
fn main() { println!("{}", sum_of_unique(vec![1, 2, 3, 2, 1])); }
#[cfg(test)]
mod tests {
    use super::sum_of_unique;
    #[test]
    fn example_one() { assert_eq!(sum_of_unique(vec![1, 2, 3, 2, 1]), 3); }
    #[test]
    fn example_two() { assert_eq!(sum_of_unique(vec![1, 1, 1, 1, 1]), 0); }
}
