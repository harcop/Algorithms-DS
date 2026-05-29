/// LeetCode #1563 - Stone Game V
fn stone_game_v(stone_value: Vec<i32>) -> i32 {
    let n = stone_value.len();
    let mut prefix = vec![0i64; n + 1];
    for (i, &v) in stone_value.iter().enumerate() {
        prefix[i + 1] = prefix[i] + v as i64;
    }
    let mut dp = vec![vec![0i32; n]; n];
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            let mut best = 0;
            for k in i..j {
                let left = prefix[k + 1] - prefix[i];
                let right = prefix[j + 1] - prefix[k + 1];
                if left <= right {
                    best = best.max(dp[i][k] + left as i32);
                }
                if left >= right {
                    best = best.max(dp[k + 1][j] + right as i32);
                }
            }
            dp[i][j] = best;
        }
    }
    dp[0][n - 1]
}

fn main() {
    println!("{}", stone_game_v(vec![6, 2, 3, 4, 5, 5]));
}

#[cfg(test)]
mod tests {
    use super::stone_game_v;

    #[test]
    fn example_one() {
        assert_eq!(stone_game_v(vec![6, 2, 3, 4, 5, 5]), 18);
    }

    #[test]
    fn example_two() {
        assert_eq!(stone_game_v(vec![4]), 0);
    }
}
