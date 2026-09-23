/// LeetCode #3891 - Minimum Increase to Maximize Special Indices
fn min_increase(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    if n < 3 {
        return 0;
    }

    type Pair = (i64, i64);
    let mut memo = vec![vec![None::<Pair>; 2]; n];

    fn dfs(i: usize, j: usize, nums: &[i32], memo: &mut [Vec<Option<Pair>>]) -> Pair {
        let n = nums.len();
        if i >= n - 1 {
            return (0, 0);
        }
        if let Some(v) = memo[i][j] {
            return v;
        }

        let left = nums[i - 1] as i64;
        let right = nums[i + 1] as i64;
        let peak_cost = (left.max(right) + 1 - nums[i] as i64).max(0);

        let take = {
            let (p, c) = dfs(i + 2, j, nums, memo);
            (p + 1, c + peak_cost)
        };

        let mut best = take;
        if j > 0 {
            let skip = dfs(i + 1, 0, nums, memo);
            if skip.0 > best.0 || (skip.0 == best.0 && skip.1 < best.1) {
                best = skip;
            }
        }

        memo[i][j] = Some(best);
        best
    }

    dfs(1, (n & 1) ^ 1, &nums, &mut memo).1
}

fn main() {
    println!("{}", min_increase(vec![1, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::min_increase;

    #[test]
    fn example1() {
        assert_eq!(min_increase(vec![1, 2, 2]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_increase(vec![2, 1, 1, 3]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(min_increase(vec![5, 2, 1, 4, 3]), 4);
    }
}
