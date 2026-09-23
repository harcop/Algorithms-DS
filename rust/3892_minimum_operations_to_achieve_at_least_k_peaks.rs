/// LeetCode #3892 - Minimum Operations to Achieve at Least K Peaks
fn min_operations(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let n = nums.len();
    if k == 0 {
        return 0;
    }
    if 2 * k > n {
        return -1;
    }

    const INF: i64 = i64::MAX / 4;

    let peak_cost = |i: usize| -> i64 {
        let l = nums[(i + n - 1) % n] as i64;
        let r = nums[(i + 1) % n] as i64;
        (l.max(r) + 1 - nums[i] as i64).max(0)
    };

    let solve = |start: usize, end: usize| -> i64 {
        if end <= start {
            return if k == 0 { 0 } else { INF };
        }
        let mut dp = vec![vec![INF; k + 1]; end + 2];
        for need in 0..=k {
            dp[end][need] = if need == 0 { 0 } else { INF };
            dp[end + 1][need] = if need == 0 { 0 } else { INF };
        }
        for i in (start..end).rev() {
            for need in 0..=k {
                let mut best = dp[i + 1][need];
                if need > 0 {
                    let tail = if i + 2 <= end {
                        dp[i + 2][need - 1]
                    } else if need == 1 {
                        0
                    } else {
                        INF
                    };
                    best = best.min(peak_cost(i) + tail);
                }
                dp[i][need] = best;
            }
        }
        dp[start][k]
    };

    let a = solve(0, n - 1);
    let b = solve(1, n);
    let ans = a.min(b);
    if ans >= INF {
        -1
    } else {
        ans
    }
}

fn main() {
    println!("{}", min_operations(vec![2, 1, 2], 1));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![2, 1, 2], 1), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![4, 5, 3, 6], 2), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(vec![3, 7, 3], 2), -1);
    }
}
