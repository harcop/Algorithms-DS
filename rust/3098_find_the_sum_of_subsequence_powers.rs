/// LeetCode #3098 - Find the Sum of Subsequence Powers
use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;
const INF: i32 = i32::MAX;

fn sum_of_powers(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let mut memo: HashMap<(usize, usize, i32, i32), i32> = HashMap::new();

    fn dfs(
        i: usize,
        j: usize,
        k: i32,
        mi: i32,
        nums: &[i32],
        n: usize,
        memo: &mut HashMap<(usize, usize, i32, i32), i32>,
    ) -> i32 {
        if i >= n {
            return if k == 0 { mi } else { 0 };
        }
        if ((n - i) as i32) < k {
            return 0;
        }
        let key = (i, j, k, mi);
        if let Some(&v) = memo.get(&key) {
            return v;
        }
        let mut ans = dfs(i + 1, j, k, mi, nums, n, memo) as i64;
        if j == n {
            ans += dfs(i + 1, i, k - 1, mi, nums, n, memo) as i64;
        } else {
            ans += dfs(i + 1, i, k - 1, mi.min(nums[i] - nums[j]), nums, n, memo) as i64;
        }
        ans %= MOD;
        let ans = ans as i32;
        memo.insert(key, ans);
        ans
    }

    dfs(0, n, k, INF, &nums, n, &mut memo)
}

fn main() {
    println!("{}", sum_of_powers(vec![1, 2, 3, 4], 3));
}

#[cfg(test)]
mod tests {
    use super::sum_of_powers;

    #[test]
    fn example1() {
        assert_eq!(sum_of_powers(vec![1, 2, 3, 4], 3), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(sum_of_powers(vec![2, 2], 2), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(sum_of_powers(vec![4, 3, -1], 2), 10);
    }
}
