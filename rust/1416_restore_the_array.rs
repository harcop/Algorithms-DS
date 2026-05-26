/// LeetCode #1416 - Restore The Array
fn number_of_arrays(s: String, k: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let b = s.as_bytes();
    let n = b.len();
    let k = k as i64;
    let mut dp = vec![0i32; n + 1];
    dp[0] = 1;
    for i in 0..n {
        if dp[i] == 0 {
            continue;
        }
        if b[i] == b'0' {
            continue;
        }
        let mut val = 0i64;
        for j in i..n {
            val = val * 10 + (b[j] - b'0') as i64;
            if val > k {
                break;
            }
            dp[j + 1] = (dp[j + 1] + dp[i]) % MOD;
        }
    }
    dp[n]
}

fn main() {
    println!("{}", number_of_arrays("1000".into(), 10));
}

#[cfg(test)]
mod tests {
    use super::number_of_arrays;

    #[test]
    fn example_one() {
        assert_eq!(number_of_arrays("1000".into(), 10000), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_arrays("1000".into(), 10), 0);
    }
}

