/// LeetCode #3704 - Count No-Zero Pairs That Sum to N
fn count_no_zero_pairs(mut n: i64) -> i64 {
    // dp[carry][a_finished][b_finished]
    let mut dp = [[[0i64; 2]; 2]; 2];
    dp[0][0][0] = 1;
    let mut start = 1i64;
    while n > 0 {
        let d = n % 10;
        n /= 10;
        let mut new_dp = [[[0i64; 2]; 2]; 2];
        for c in 0..2 {
            for i in 0..2 {
                for j in 0..2 {
                    let ways = dp[c][i][j];
                    if ways == 0 {
                        continue;
                    }
                    let xmax = if i == 0 { 9 } else { 0 };
                    for x in start..=xmax {
                        let ymax = if j == 0 { 9 } else { 0 };
                        for y in start..=ymax {
                            if (c as i64 + x + y) % 10 != d {
                                continue;
                            }
                            let nc = ((c as i64 + x + y) / 10) as usize;
                            let ni = if i == 1 || x == 0 { 1 } else { 0 };
                            let nj = if j == 1 || y == 0 { 1 } else { 0 };
                            new_dp[nc][ni][nj] += ways;
                        }
                    }
                }
            }
        }
        start = 0;
        dp = new_dp;
    }
    let mut ans = 0i64;
    for i in 0..2 {
        for j in 0..2 {
            ans += dp[0][i][j];
        }
    }
    ans
}

fn main() {
    println!("{}", count_no_zero_pairs(11));
}

#[cfg(test)]
mod tests {
    use super::count_no_zero_pairs;

    #[test]
    fn example1() {
        assert_eq!(count_no_zero_pairs(2), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(count_no_zero_pairs(3), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(count_no_zero_pairs(11), 8);
    }
}
