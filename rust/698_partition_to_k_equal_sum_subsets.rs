/// LeetCode #698 - Partition to K Equal Sum Subsets
use std::collections::HashMap;

fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
    let total: i32 = nums.iter().sum();
    if total % k != 0 {
        return false;
    }
    let target = total / k;
    let mut nums = nums;
    nums.sort_unstable_by(|a, b| b.cmp(a));
    if nums[0] > target {
        return false;
    }
    let n = nums.len();
    let full = if n == 32 { u32::MAX } else { (1u32 << n) - 1 };
    let mut memo: HashMap<u32, bool> = HashMap::new();

    fn dfs(
        mask: u32,
        rem: i32,
        target: i32,
        nums: &[i32],
        full: u32,
        memo: &mut HashMap<u32, bool>,
    ) -> bool {
        if mask == full {
            return true;
        }
        if let Some(&v) = memo.get(&mask) {
            return v;
        }
        for i in 0..nums.len() {
            if mask & (1u32 << i) != 0 {
                continue;
            }
            if rem + nums[i] > target {
                continue;
            }
            let new_mask = mask | (1u32 << i);
            let new_rem = (rem + nums[i]) % target;
            if dfs(new_mask, new_rem, target, nums, full, memo) {
                memo.insert(mask, true);
                return true;
            }
        }
        memo.insert(mask, false);
        false
    }

    dfs(0, 0, target, &nums, full, &mut memo)
}

fn main() {
    println!(
        "{}",
        can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4)
    );
}

#[cfg(test)]
mod tests {
    use super::can_partition_k_subsets;

    #[test]
    fn example_one() {
        assert!(can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4));
    }

    #[test]
    fn example_two() {
        assert!(!can_partition_k_subsets(vec![1, 2, 3, 4], 3));
    }
}
