/// LeetCode #1406 - Stone Game Iii
fn stone_game_iii(stone_value: Vec<i32>) -> String {
    let n = stone_value.len();
    let mut dp = vec![0i64; n + 1];
    for i in (0..n).rev() {
        let mut best = i64::MIN / 4;
        let mut take = 0i64;
        for j in 0..3 {
            if i + j < n {
                take += stone_value[i + j] as i64;
                best = best.max(take - dp[i + j + 1]);
            }
        }
        dp[i] = best;
    }
    if dp[0] > 0 {
        "Alice".into()
    } else if dp[0] < 0 {
        "Bob".into()
    } else {
        "Tie".into()
    }
}

fn main() {
    println!("{}", stone_game_iii(vec![1, 2, 3, 6]));
}

#[cfg(test)]
mod tests {
    use super::stone_game_iii;

    #[test]
    fn example_one() {
        assert_eq!(stone_game_iii(vec![1, 2, 3, 7]), "Bob");
    }

    #[test]
    fn example_two() {
        assert_eq!(stone_game_iii(vec![1, 2, 3, 6]), "Tie");
    }
}

