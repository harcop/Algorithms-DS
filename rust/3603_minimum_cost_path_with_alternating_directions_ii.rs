/// LeetCode #3603 - Minimum Cost Path with Alternating Directions II
fn min_cost(m: i32, n: i32, wait_cost: Vec<Vec<i32>>) -> i64 {
    let m = m as usize;
    let n = n as usize;
    let mut dp = vec![0i64; n];
    for i in 0..m {
        for j in 0..n {
            let wait = if (i == 0 && j == 0) || (i == m - 1 && j == n - 1) {
                0
            } else {
                wait_cost[i][j] as i64
            };
            let mut prev = if i == 0 && j == 0 { 0i64 } else { i64::MAX / 4 };
            if i > 0 {
                prev = prev.min(dp[j]);
            }
            if j > 0 {
                prev = prev.min(dp[j - 1]);
            }
            dp[j] = prev + wait + (i as i64 + 1) * (j as i64 + 1);
        }
    }
    dp[n - 1]
}

fn main() {
    println!("{}", min_cost(1, 2, vec![vec![1, 2]]));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example1() {
        assert_eq!(min_cost(1, 2, vec![vec![1, 2]]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(min_cost(2, 2, vec![vec![3, 5], vec![2, 4]]), 9);
    }

    #[test]
    fn example3() {
        assert_eq!(min_cost(2, 3, vec![vec![6, 1, 4], vec![3, 2, 5]]), 16);
    }
}
