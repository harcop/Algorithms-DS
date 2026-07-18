/// LeetCode #2465 - Number of Distinct Averages
use std::collections::HashSet;

fn distinct_averages(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let mut sums = HashSet::new();

    for i in 0..n / 2 {
        sums.insert(nums[i] + nums[n - 1 - i]);
    }

    sums.len() as i32
}

fn main() {
    println!("{}", distinct_averages(vec![4, 1, 4, 0, 3, 5]));
}

#[cfg(test)]
mod tests {
    use super::distinct_averages;

    #[test]
    fn example_one() {
        assert_eq!(distinct_averages(vec![4, 1, 4, 0, 3, 5]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(distinct_averages(vec![1, 100]), 1);
    }
}
