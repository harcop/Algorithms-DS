/// LeetCode #152 - Maximum Product Subarray
fn max_product(nums: Vec<i32>) -> i32 {
    let mut max_p = nums[0];
    let mut min_p = nums[0];
    let mut ans = nums[0];
    for &x in nums.iter().skip(1) {
        if x < 0 {
            std::mem::swap(&mut max_p, &mut min_p);
        }
        max_p = x.max(max_p * x);
        min_p = x.min(min_p * x);
        ans = ans.max(max_p);
    }
    ans
}

fn main() {
    println!("{}", max_product(vec![2, 3, -2, 4]));
}

#[cfg(test)]
mod tests {
    use super::max_product;

    #[test]
    fn example_one() {
        assert_eq!(max_product(vec![2, 3, -2, 4]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_product(vec![-2, 0, -1]), 0);
    }
}
