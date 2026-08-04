/// LeetCode #2958 - Length of Longest Subarray With at Most K Frequency
use std::collections::HashMap;

fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut ans = 0;
    let mut j = 0usize;
    for (i, &x) in nums.iter().enumerate() {
        *cnt.entry(x).or_insert(0) += 1;
        while *cnt.get(&x).unwrap() > k {
            *cnt.get_mut(&nums[j]).unwrap() -= 1;
            j += 1;
        }
        ans = ans.max((i - j + 1) as i32);
    }
    ans
}

fn main() {
    println!("{}", max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2));
}

#[cfg(test)]
mod tests {
    use super::max_subarray_length;

    #[test]
    fn example_one() {
        assert_eq!(max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_subarray_length(vec![1, 2, 1, 2, 1, 2, 1, 2], 1), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_subarray_length(vec![5, 5, 5, 5, 5, 5, 5], 4), 4);
    }
}
