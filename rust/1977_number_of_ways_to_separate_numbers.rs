/// LeetCode #1977 - Number of Ways to Separate Numbers
const MOD: i64 = 1_000_000_007;

fn number_of_combinations(num: String) -> i32 {
    let num = num.as_bytes();
    let n = num.len();
    if n == 0 || num[0] == b'0' {
        return 0;
    }

    let mut lcp = vec![vec![0usize; n + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..n).rev() {
            if num[i] == num[j] {
                lcp[i][j] = 1 + lcp[i + 1][j + 1];
            }
        }
    }

    let cmp = |i: usize, j: usize, k: usize| -> bool {
        let x = lcp[i][j];
        x >= k || num[i + x] >= num[j + x]
    };

    let mut dp = vec![vec![0i64; n + 1]; n + 1];
    dp[0][0] = 1;

    for i in 1..=n {
        for j in 1..=i {
            let mut v = 0i64;
            if num[i - j] != b'0' {
                let prev = i - j;
                if prev >= j && cmp(prev, prev - j, j) {
                    v = dp[prev][j];
                } else {
                    v = dp[prev][(j - 1).min(prev)];
                }
            }
            dp[i][j] = (dp[i][j - 1] + v) % MOD;
        }
    }
    dp[n][n] as i32
}

fn main() {
    println!("{}", number_of_combinations("327".into()));
}

#[cfg(test)]
mod tests {
    use super::number_of_combinations;

    #[test]
    fn example_one() {
        assert_eq!(number_of_combinations("327".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_combinations("094".into()), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(number_of_combinations("0".into()), 0);
    }
}
