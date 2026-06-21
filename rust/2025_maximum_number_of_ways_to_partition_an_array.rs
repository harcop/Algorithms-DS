/// LeetCode #2025 - Maximum Number of Ways to Partition an Array
use std::collections::HashMap;

fn ways_to_partition(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut prefix = vec![nums[0]; n];
    for i in 1..n {
        prefix[i] = prefix[i - 1] + nums[i];
    }

    let mut right = HashMap::new();
    for i in 1..n {
        *right.entry(prefix[i - 1]).or_insert(0) += 1;
    }

    let total = prefix[n - 1];
    let mut ans = if total % 2 == 0 {
        *right.get(&(total / 2)).unwrap_or(&0)
    } else {
        0
    };

    let mut left = HashMap::new();
    for i in 0..n {
        let d = k - nums[i];
        if (total + d) % 2 == 0 {
            let t = left.get(&((total + d) / 2)).copied().unwrap_or(0)
                + right.get(&((total - d) / 2)).copied().unwrap_or(0);
            ans = ans.max(t);
        }
        *left.entry(prefix[i]).or_insert(0) += 1;
        if let Some(v) = right.get_mut(&prefix[i]) {
            *v -= 1;
        }
    }
    ans
}

fn main() {
    println!("{}", ways_to_partition(vec![2, -1, 2], 3));
}

#[cfg(test)]
mod tests {
    use super::ways_to_partition;

    #[test]
    fn example_one() {
        assert_eq!(ways_to_partition(vec![2, -1, 2], 3), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(ways_to_partition(vec![0, 0, 0], 1), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            ways_to_partition(
                vec![22, 4, -25, -20, -15, 15, -16, 7, 19, -10, 0, -13, -14],
                -33
            ),
            4
        );
    }
}
