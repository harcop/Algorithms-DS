/// LeetCode #3500 - Minimum Cost to Divide Array Into Subarrays
fn minimum_cost(nums: Vec<i32>, cost: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let mut prefix_nums = vec![0i64; n + 1];
    let mut prefix_cost = vec![0i64; n + 1];
    for i in 0..n {
        prefix_nums[i + 1] = prefix_nums[i] + nums[i] as i64;
        prefix_cost[i + 1] = prefix_cost[i] + cost[i] as i64;
    }
    let k = k as i64;
    const INF: i64 = i64::MAX / 4;
    let mut dp = vec![INF; n + 1];
    dp[n] = 0;
    for i in (0..n).rev() {
        for j in i..n {
            dp[i] = dp[i].min(
                prefix_nums[j + 1] * (prefix_cost[j + 1] - prefix_cost[i])
                    + k * (prefix_cost[n] - prefix_cost[i])
                    + dp[j + 1],
            );
        }
    }
    dp[0]
}

fn main() {
    println!("{}", minimum_cost(vec![3, 1, 4], vec![4, 6, 6], 1));
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example1() {
        assert_eq!(minimum_cost(vec![3, 1, 4], vec![4, 6, 6], 1), 110);
    }

    #[test]
    fn example2() {
        assert_eq!(
            minimum_cost(vec![4, 8, 5, 1, 14, 2, 2, 12, 1], vec![7, 2, 8, 4, 2, 2, 1, 1, 2], 7),
            985
        );
    }
}
