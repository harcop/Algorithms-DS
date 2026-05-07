/// LeetCode #375 - Guess Number Higher or Lower II (minimax DP)
fn get_money_amount(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![vec![0i32; n + 2]; n + 2];
    for len in 2..=n {
        for i in 1..=n + 1 - len {
            let j = i + len - 1;
            dp[i][j] = i32::MAX;
            for x in i..=j {
                let cost =
                    x as i32 + dp[i][x - 1].max(dp[x + 1][j]);
                dp[i][j] = dp[i][j].min(cost);
            }
        }
    }
    dp[1][n]
}

fn main() {
    println!("{}", get_money_amount(10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc() {
        assert_eq!(get_money_amount(1), 0);
        assert_eq!(get_money_amount(2), 1);
        assert_eq!(get_money_amount(3), 2);
        assert_eq!(get_money_amount(4), 4);
    }
}
