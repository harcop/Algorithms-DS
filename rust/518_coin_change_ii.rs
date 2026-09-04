/// LeetCode #518 - Coin Change II
fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let amount = amount as usize;
    let mut dp = vec![0; amount + 1];
    dp[0] = 1;
    for c in coins {
        let c = c as usize;
        for a in c..=amount {
            dp[a] += dp[a - c];
        }
    }
    dp[amount]
}

fn main() {
    println!("{}", change(5, vec![1, 2, 5]));
}

#[cfg(test)]
mod tests {
    use super::change;

    #[test]
    fn example_one() {
        assert_eq!(change(5, vec![1, 2, 5]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(change(3, vec![2]), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(change(10, vec![10]), 1);
    }
}
