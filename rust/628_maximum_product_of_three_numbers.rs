/// LeetCode #628 - Maximum Product of Three Numbers
fn maximum_product(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let a = nums[n - 1] * nums[n - 2] * nums[n - 3];
    let b = nums[0] * nums[1] * nums[n - 1];
    a.max(b)
}

fn main() {
    println!("{}", maximum_product(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::maximum_product;

    #[test]
    fn example_one() {
        assert_eq!(maximum_product(vec![1, 2, 3, 4]), 24);
    }

    #[test]
    fn negatives() {
        assert_eq!(maximum_product(vec![-10, -10, 5, 2]), 500);
    }
}
