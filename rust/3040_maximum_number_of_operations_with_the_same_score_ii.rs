/// LeetCode #3040 - Maximum Number of Operations With the Same Score II
use std::collections::HashMap;

fn dfs(
    i: usize,
    j: usize,
    s: i32,
    nums: &[i32],
    memo: &mut HashMap<(usize, usize, i32), i32>,
) -> i32 {
    if j <= i {
        return 0;
    }
    if let Some(&v) = memo.get(&(i, j, s)) {
        return v;
    }
    let mut ans = 0;
    if nums[i] + nums[i + 1] == s {
        ans = ans.max(1 + dfs(i + 2, j, s, nums, memo));
    }
    if nums[i] + nums[j] == s {
        ans = ans.max(1 + dfs(i + 1, j - 1, s, nums, memo));
    }
    if nums[j - 1] + nums[j] == s {
        ans = ans.max(1 + dfs(i, j - 2, s, nums, memo));
    }
    memo.insert((i, j, s), ans);
    ans
}

fn max_operations(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n < 2 {
        return 0;
    }
    let mut memo = HashMap::new();
    let s1 = nums[0] + nums[1];
    let s2 = nums[n - 2] + nums[n - 1];
    let s3 = nums[0] + nums[n - 1];
    1 + dfs(2, n - 1, s1, &nums, &mut memo)
        .max(dfs(0, n - 3, s2, &nums, &mut memo))
        .max(dfs(1, n - 2, s3, &nums, &mut memo))
}

fn main() {
    println!("{}", max_operations(vec![3, 2, 1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::max_operations;

    #[test]
    fn example1() {
        assert_eq!(max_operations(vec![3, 2, 1, 2, 3, 4]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(max_operations(vec![3, 2, 6, 1, 4]), 2);
    }
}
