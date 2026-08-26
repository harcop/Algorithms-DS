/// LeetCode #3429 - Paint House IV
fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> i64 {
    let n = n as usize;
    let half = n / 2;
    let inf = i64::MAX / 4;
    let mut dp = vec![vec![0i64; 3]; 3];
    for i in (0..half).rev() {
        let mut ndp = vec![vec![inf; 3]; 3];
        for lc in 0..3 {
            for rc in 0..3 {
                if lc == rc {
                    continue;
                }
                let mut best = inf;
                if i + 1 == half {
                    best = 0;
                } else {
                    for nlc in 0..3 {
                        if nlc == lc {
                            continue;
                        }
                        for nrc in 0..3 {
                            if nrc == rc {
                                continue;
                            }
                            best = best.min(dp[nlc][nrc]);
                        }
                    }
                }
                ndp[lc][rc] = cost[i][lc] as i64 + cost[n - 1 - i][rc] as i64 + best;
            }
        }
        dp = ndp;
    }
    let mut ans = inf;
    for lc in 0..3 {
        for rc in 0..3 {
            ans = ans.min(dp[lc][rc]);
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        min_cost(
            4,
            vec![vec![3, 5, 7], vec![6, 2, 9], vec![4, 8, 1], vec![7, 3, 5]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example1() {
        assert_eq!(
            min_cost(
                4,
                vec![vec![3, 5, 7], vec![6, 2, 9], vec![4, 8, 1], vec![7, 3, 5]]
            ),
            9
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_cost(
                6,
                vec![
                    vec![2, 4, 6],
                    vec![5, 3, 8],
                    vec![7, 1, 9],
                    vec![4, 6, 2],
                    vec![3, 5, 7],
                    vec![8, 2, 4]
                ]
            ),
            18
        );
    }
}
