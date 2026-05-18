/// LeetCode #918 - Maximum Sum Circular Subarray
fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let total: i32 = nums.iter().sum();
    let mut max_k = nums[0];
    let mut cur = nums[0];
    for i in 1..n {
        cur = nums[i].max(cur + nums[i]);
        max_k = max_k.max(cur);
    }
    let mut min_k = nums[0];
    cur = nums[0];
    for i in 1..n {
        cur = nums[i].min(cur + nums[i]);
        min_k = min_k.min(cur);
    }
    if max_k < 0 {
        max_k
    } else {
        max_k.max(total - min_k)
    }
}

fn main() {
    println!("{}", max_subarray_sum_circular(vec![1, -2, 3, -2]));
}

#[cfg(test)]
mod tests {
    use super::max_subarray_sum_circular;

    #[test]
    fn example_one() {
        assert_eq!(max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_subarray_sum_circular(vec![5, -3, 5]), 10);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_subarray_sum_circular(vec![-3, -2, -3]), -2);
    }
}
