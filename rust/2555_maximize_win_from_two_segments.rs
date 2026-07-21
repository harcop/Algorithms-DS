/// LeetCode #2555 - Maximize Win From Two Segments
fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
    let n = prize_positions.len();
    let mut dp = vec![0; n + 1];
    let mut ans = 0;
    let mut j = 0usize;

    for i in 0..n {
        while prize_positions[i] - prize_positions[j] > k {
            j += 1;
        }
        let covered = (i - j + 1) as i32;
        dp[i + 1] = dp[i].max(covered);
        ans = ans.max(dp[j] + covered);
    }
    ans
}

fn main() {
    println!("{}", maximize_win(vec![1, 1, 2, 2, 3, 3, 5], 2));
}

#[cfg(test)]
mod tests {
    use super::maximize_win;

    #[test]
    fn example_one() {
        assert_eq!(maximize_win(vec![1, 1, 2, 2, 3, 3, 5], 2), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximize_win(vec![1, 2, 3, 4], 0), 2);
    }
}
