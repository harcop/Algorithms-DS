/// LeetCode #920 - Number of Music Playlists
const MOD920: i64 = 1_000_000_007;

fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
    let n = n as usize;
    let goal = goal as usize;
    let k = k as usize;
    let mut dp = vec![vec![0i64; n + 1]; goal + 1];
    dp[0][0] = 1;
    for i in 1..=goal {
        for j in 1..=n.min(i) {
            dp[i][j] = dp[i - 1][j - 1] * ((n - (j - 1)) as i64) % MOD920;
            if j > k {
                dp[i][j] = (dp[i][j] + dp[i - 1][j] * ((j - k) as i64)) % MOD920;
            }
        }
    }
    dp[goal][n] as i32
}

fn main() {
    println!("{}", num_music_playlists(3, 3, 1));
}

#[cfg(test)]
mod tests {
    use super::num_music_playlists;

    #[test]
    fn example_one() {
        assert_eq!(num_music_playlists(3, 3, 1), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_music_playlists(2, 3, 0), 6);
    }

    #[test]
    fn example_three() {
        assert_eq!(num_music_playlists(2, 3, 1), 2);
    }
}
