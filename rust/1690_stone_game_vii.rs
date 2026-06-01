/// LeetCode #1690 - Stone Game Vii
fn stone_game_vii(stones: Vec<i32>) -> i32 {
    let n = stones.len();
    let mut pref = vec![0i32; n + 1];
    for i in 0..n { pref[i + 1] = pref[i] + stones[i]; }
    let mut dp = vec![vec![0i32; n]; n];
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            let sum = pref[j + 1] - pref[i];
            dp[i][j] = (sum - stones[i] - dp[i + 1][j]).max(sum - stones[j] - dp[i][j - 1]);
        }
    }
    dp[0][n - 1]
}
fn main() { println!("{}", stone_game_vii(vec![5,3,1,4,2])); }
#[cfg(test)]
mod tests {
    use super::stone_game_vii;
    #[test]
    fn example_one() { assert_eq!(stone_game_vii(vec![5,3,1,4,2]), 6); }
}