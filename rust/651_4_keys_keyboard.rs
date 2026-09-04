/// LeetCode #651 - 4 Keys Keyboard
fn max_a(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0i32; n + 1];
    for i in 1..=n {
        dp[i] = dp[i - 1] + 1;
        for j in 0..i.saturating_sub(2) {
            dp[i] = dp[i].max(dp[j] * (i as i32 - j as i32 - 1));
        }
    }
    dp[n]
}

fn main() {
    println!("{}", max_a(7));
}

#[cfg(test)]
mod tests {
    use super::max_a;

    #[test]
    fn example_one() {
        assert_eq!(max_a(3), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_a(7), 9);
    }
}
