/// LeetCode #1913 - Maximum Product Difference Between Two Pairs
fn max_product_difference(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    nums[n - 1] * nums[n - 2] - nums[0] * nums[1]
}

fn main() {
    println!("{}", max_product_difference(vec![5, 6, 2, 7, 4]));
}

#[cfg(test)]
mod tests {
    use super::max_product_difference;

    #[test]
    fn example_one() {
        assert_eq!(max_product_difference(vec![5, 6, 2, 7, 4]), 34);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_product_difference(vec![4, 2, 5, 9, 7, 4, 8]), 64);
    }
}
