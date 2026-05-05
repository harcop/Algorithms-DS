/// LeetCode #322 - Coin Change
fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let a = amount as usize;
    let mut dp = vec![amount + 1; a + 1];
    dp[0] = 0;
    for i in 1..=a {
        for &c in &coins {
            if c as usize <= i {
                dp[i] = dp[i].min(dp[i - c as usize] + 1);
            }
        }
    }
    if dp[a] > amount { -1 } else { dp[a] }
}

fn main() {
    println!("{}", coin_change(vec![1,2,5], 11));
}

#[cfg(test)]
mod tests {
    use super::coin_change;

    #[test]
    fn example_one() {
        assert_eq!(coin_change(vec![1,2,5], 11), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(coin_change(vec![2], 3), -1);
    }
}
