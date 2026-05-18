/// LeetCode #1000 - Minimum Cost to Merge Stones
fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
    let n = stones.len();
    let k = k as usize;
    if (n - 1) % (k - 1) != 0 {
        return -1;
    }
    let mut prefix = vec![0i64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + stones[i] as i64;
    }
    let sum = |l: usize, r: usize| -> i64 { prefix[r + 1] - prefix[l] };
    let mut dp = vec![vec![vec![0i64; k + 1]; n]; n];
    for i in 0..n {
        dp[i][i][1] = 0;
    }
    for len in 2..=n {
        for l in 0..=n - len {
            let r = l + len - 1;
            for m in 2..=k.min(len) {
                dp[l][r][m] = i64::MAX / 4;
                for mid in l..r {
                    for t in 1..m {
                        if mid - l + 1 < t || r - mid < m - t {
                            continue;
                        }
                        dp[l][r][m] = dp[l][r][m].min(dp[l][mid][t] + dp[mid + 1][r][m - t]);
                    }
                }
            }
            if len >= k && dp[l][r][k] < i64::MAX / 4 {
                dp[l][r][1] = dp[l][r][k] + sum(l, r);
            }
        }
    }
    if dp[0][n - 1][1] >= i64::MAX / 4 { -1 } else { dp[0][n - 1][1] as i32 }
}

fn main() {
    println!("{}", merge_stones(vec![3, 2, 4, 1], 2));
}

#[cfg(test)]
mod tests {
    use super::merge_stones;

    #[test]
    fn example_one() {
        assert_eq!(merge_stones(vec![3, 2, 4, 1], 2), 20);
    }

    #[test]
    fn example_two() {
        assert_eq!(merge_stones(vec![3, 2, 4, 1], 3), -1);
    }
}
