/// LeetCode #3980 - Minimum Operations to Transform Binary String
fn min_operations(s1: String, s2: String) -> i32 {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let n = s1.len();
    let inf = i32::MAX / 4;
    let mut dp = vec![[inf; 2]; n + 1];
    dp[n][0] = 0;
    dp[n][1] = 0;
    for i in (0..n).rev() {
        for cleared in 0..2 {
            let cur = if cleared == 1 { b'0' } else { s1[i] };
            let tgt = s2[i];
            let mut best = inf;
            if cur == tgt {
                best = best.min(dp[i + 1][0]);
            } else if cur == b'0' && tgt == b'1' {
                best = best.min(1 + dp[i + 1][0]);
            } else if cur == b'1' && tgt == b'0' && i > 0 {
                best = best.min(2 + dp[i + 1][0]);
            }
            if i + 1 < n {
                let nxt = s1[i + 1];
                let mut cost = 1;
                if cur == b'0' {
                    cost += 1;
                }
                if nxt == b'0' {
                    cost += 1;
                }
                if tgt == b'1' {
                    cost += 1;
                }
                let nxt_cleared = if nxt == b'1' { 1 } else { 0 };
                best = best.min(cost + dp[i + 1][nxt_cleared]);
            }
            dp[i][cleared] = best;
        }
    }
    if dp[0][0] >= inf {
        -1
    } else {
        dp[0][0]
    }
}

fn main() {
    println!("{}", min_operations("11".into(), "00".into()));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations("11".into(), "00".into()), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations("01".into(), "10".into()), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations("1".into(), "0".into()), -1);
    }
}
