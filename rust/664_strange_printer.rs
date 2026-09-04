/// LeetCode #664 - Strange Printer
fn strange_printer(s: String) -> i32 {
    let s: Vec<u8> = s.bytes().collect();
    let n = s.len();
    if n == 0 {
        return 0;
    }
    let mut dp = vec![vec![0i32; n]; n];
    for i in 0..n {
        dp[i][i] = 1;
    }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            dp[i][j] = dp[i][j - 1] + 1;
            for k in i..j {
                if s[k] == s[j] {
                    let right = if k + 1 <= j - 1 { dp[k + 1][j - 1] } else { 0 };
                    dp[i][j] = dp[i][j].min(dp[i][k] + right);
                }
            }
        }
    }
    dp[0][n - 1]
}

fn main() {
    println!("{}", strange_printer("aaabbb".into()));
}

#[cfg(test)]
mod tests {
    use super::strange_printer;

    #[test]
    fn example_one() {
        assert_eq!(strange_printer("aaabbb".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(strange_printer("aba".into()), 2);
    }
}
