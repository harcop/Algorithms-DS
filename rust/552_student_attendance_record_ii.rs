/// LeetCode #552 - Student Attendance Record II
fn check_record(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as usize;
    // dp[a][l]: absences used, consecutive lates
    let mut dp = [[0i64; 3]; 2];
    dp[0][0] = 1;
    for _ in 0..n {
        let mut ndp = [[0i64; 3]; 2];
        for a in 0..2 {
            for l in 0..3 {
                let v = dp[a][l];
                if v == 0 {
                    continue;
                }
                // P
                ndp[a][0] = (ndp[a][0] + v) % MOD;
                // A
                if a + 1 < 2 {
                    ndp[a + 1][0] = (ndp[a + 1][0] + v) % MOD;
                }
                // L
                if l + 1 < 3 {
                    ndp[a][l + 1] = (ndp[a][l + 1] + v) % MOD;
                }
            }
        }
        dp = ndp;
    }
    let mut ans = 0i64;
    for a in 0..2 {
        for l in 0..3 {
            ans = (ans + dp[a][l]) % MOD;
        }
    }
    ans as i32
}

fn main() {
    println!("{}", check_record(2));
}

#[cfg(test)]
mod tests {
    use super::check_record;

    #[test]
    fn example_one() {
        assert_eq!(check_record(2), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(check_record(1), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(check_record(10101), 183236316);
    }
}
