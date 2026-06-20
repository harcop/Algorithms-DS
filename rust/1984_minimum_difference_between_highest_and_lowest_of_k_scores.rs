/// LeetCode #1984 - Minimum Difference Between Highest and Lowest of K Scores
fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let k = k as usize;
    (0..=nums.len() - k)
        .map(|i| nums[i + k - 1] - nums[i])
        .min()
        .unwrap()
}

fn main() {
    println!("{}", minimum_difference(vec![90], 1));
}

#[cfg(test)]
mod tests {
    use super::minimum_difference;

    #[test]
    fn example_one() {
        assert_eq!(minimum_difference(vec![90], 1), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_difference(vec![9, 4, 1, 7], 2), 2);
    }
}
