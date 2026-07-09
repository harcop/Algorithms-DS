/// LeetCode #2318 - Number of Distinct Roll Sequences
fn distinct_sequences(n: i32) -> i32 {
    let n = n as usize;
    if n == 1 {
        return 6;
    }
    const MOD: i32 = 1_000_000_007;
    let mut dp = vec![[[0i32; 6]; 6]; n + 1];

    for i in 0..6usize {
        for j in 0..6usize {
            if gcd((i + 1) as i32, (j + 1) as i32) == 1 && i != j {
                dp[2][i][j] = 1;
            }
        }
    }

    for k in 3..=n {
        for i in 0..6usize {
            for j in 0..6usize {
                if gcd((i + 1) as i32, (j + 1) as i32) == 1 && i != j {
                    for h in 0..6usize {
                        if gcd((h + 1) as i32, (i + 1) as i32) == 1 && h != i && h != j {
                            dp[k][i][j] = (dp[k][i][j] + dp[k - 1][h][i]) % MOD;
                        }
                    }
                }
            }
        }
    }

    let mut ans = 0i32;
    for i in 0..6usize {
        for j in 0..6usize {
            ans = (ans + dp[n][i][j]) % MOD;
        }
    }
    ans
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn main() {
    println!("{}", distinct_sequences(4));
}

#[cfg(test)]
mod tests {
    use super::distinct_sequences;

    #[test]
    fn example_one() {
        assert_eq!(distinct_sequences(4), 184);
    }

    #[test]
    fn example_two() {
        assert_eq!(distinct_sequences(2), 22);
    }
}
