/// LeetCode #3073 - Maximum Increasing Triplet Value
use std::collections::BTreeSet;

fn maximum_triplet_value(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut right = vec![0; n];
    right[n - 1] = nums[n - 1];
    for i in (0..n - 1).rev() {
        right[i] = right[i + 1].max(nums[i]);
    }

    let mut left: BTreeSet<i32> = BTreeSet::new();
    left.insert(nums[0]);
    let mut ans = i32::MIN;

    for j in 1..n - 1 {
        if right[j + 1] > nums[j] {
            if let Some(&li) = left.range(..nums[j]).next_back() {
                ans = ans.max(li - nums[j] + right[j + 1]);
            }
        }
        left.insert(nums[j]);
    }

    ans
}

fn main() {
    println!("{}", maximum_triplet_value(vec![5, 6, 9]));
}

#[cfg(test)]
mod tests {
    use super::maximum_triplet_value;

    #[test]
    fn example1() {
        assert_eq!(maximum_triplet_value(vec![5, 6, 9]), 8);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_triplet_value(vec![1, 5, 3, 6]), 4);
    }
}
