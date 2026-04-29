/// LeetCode #62 - Unique Paths
fn unique_paths(m: i32, n: i32) -> i32 {
    let (m, n) = (m as usize, n as usize);
    let mut dp = vec![1; n];
    for _ in 1..m {
        for j in 1..n {
            dp[j] += dp[j - 1];
        }
    }
    dp[n - 1]
}

fn main() {
    println!("{}", unique_paths(3, 7));
}

#[cfg(test)]
mod tests {
    use super::unique_paths;
    #[test]
    fn example_one() {
        assert_eq!(unique_paths(3, 7), 28);
    }
    #[test]
    fn example_two() {
        assert_eq!(unique_paths(3, 2), 3);
    }
}
