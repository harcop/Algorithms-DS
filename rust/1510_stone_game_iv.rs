/// LeetCode #1510 - Stone Game Iv
fn winner_square_game(n: i32) -> bool {
    let n = n as usize;
    let mut dp = vec![false; n + 1];
    for i in 1..=n {
        let mut win = false;
        let mut k = 1;
        while k * k <= i {
            if !dp[i - k * k] {
                win = true;
                break;
            }
            k += 1;
        }
        dp[i] = win;
    }
    dp[n]
}

fn main() {
    println!("{}", winner_square_game(4));
}

#[cfg(test)]
mod tests {
    use super::winner_square_game;

    #[test]
    fn example_one() {
        assert!(winner_square_game(4));
    }

    #[test]
    fn example_two() {
        assert!(winner_square_game(1));
    }
}
