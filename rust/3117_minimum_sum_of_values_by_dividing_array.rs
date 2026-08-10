/// LeetCode #3117 - Minimum Sum of Values by Dividing Array
use std::collections::HashMap;

const INF: i32 = 1 << 29;

fn minimum_value_sum(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
    let n = nums.len();
    let m = and_values.len();
    let mut memo: HashMap<(usize, usize, i32), i32> = HashMap::new();

    fn dfs(
        i: usize,
        j: usize,
        a: i32,
        nums: &[i32],
        and_values: &[i32],
        n: usize,
        m: usize,
        memo: &mut HashMap<(usize, usize, i32), i32>,
    ) -> i32 {
        if n - i < m - j {
            return INF;
        }
        if j == m {
            return if i == n { 0 } else { INF };
        }
        let a = a & nums[i];
        if a < and_values[j] {
            return INF;
        }
        let key = (i, j, a);
        if let Some(&v) = memo.get(&key) {
            return v;
        }
        let mut ans = dfs(i + 1, j, a, nums, and_values, n, m, memo);
        if a == and_values[j] {
            ans = ans.min(dfs(i + 1, j + 1, -1, nums, and_values, n, m, memo) + nums[i]);
        }
        memo.insert(key, ans);
        ans
    }

    let ans = dfs(0, 0, -1, &nums, &and_values, n, m, &mut memo);
    if ans >= INF {
        -1
    } else {
        ans
    }
}

fn main() {
    println!(
        "{}",
        minimum_value_sum(vec![1, 4, 3, 3, 2], vec![0, 3, 3, 2])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_value_sum;

    #[test]
    fn example1() {
        assert_eq!(
            minimum_value_sum(vec![1, 4, 3, 3, 2], vec![0, 3, 3, 2]),
            12
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            minimum_value_sum(vec![2, 3, 5, 7, 7, 7, 5], vec![0, 7, 5]),
            17
        );
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_value_sum(vec![1, 2, 3, 4], vec![2]), -1);
    }
}
