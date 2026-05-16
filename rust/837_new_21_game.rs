/// LeetCode #837 - New 21 Game
fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
    if k == 0 || n >= k - 1 + max_pts {
        return 1.0;
    }
    let n = n as usize;
    let max_pts = max_pts as usize;
    let mut dp = vec![0.0; n + 1];
    dp[0] = 1.0;
    let mut window = 1.0;
    let mut ans = 0.0;
    for i in 1..=n {
        dp[i] = window / max_pts as f64;
        if i < k as usize {
            window += dp[i];
        } else {
            ans += dp[i];
        }
        if i >= max_pts {
            window -= dp[i - max_pts];
        }
    }
    ans
}

fn main() {
    println!("{}", new21_game(10, 1, 10));
}

#[cfg(test)]
mod tests {
    use super::new21_game;

    #[test]
    fn example_one() {
        assert!((new21_game(10, 1, 10) - 1.0).abs() < 1e-5);
    }
}
