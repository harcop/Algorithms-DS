/// LeetCode #583 - Delete Operation for Two Strings
fn min_distance(word1: String, word2: String) -> i32 {
    let a = word1.as_bytes();
    let b = word2.as_bytes();
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]);
            }
        }
    }
    dp[m][n] as i32
}

fn main() {
    println!("{}", min_distance("sea".into(), "eat".into()));
}

#[cfg(test)]
mod tests {
    use super::min_distance;

    #[test]
    fn example_one() {
        assert_eq!(min_distance("sea".into(), "eat".into()), 2);
    }
}
