/// LeetCode #713 - Subarray Product Less Than K
fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    if k <= 1 { return 0; }
    let mut prod = 1i64;
    let mut left = 0usize;
    let mut ans = 0i32;
    for right in 0..nums.len() {
        prod *= nums[right] as i64;
        while prod >= k as i64 {
            prod /= nums[left] as i64;
            left += 1;
        }
        ans += (right - left + 1) as i32;
    }
    ans
}

fn main() {
    println!("{}", num_subarray_product_less_than_k(vec![10,5,2,6], 100));
}

#[cfg(test)]
mod tests {
    use super::num_subarray_product_less_than_k;

    #[test]
    fn example_one() {
        assert_eq!(num_subarray_product_less_than_k(vec![10,5,2,6], 100), 8);
    }
}
