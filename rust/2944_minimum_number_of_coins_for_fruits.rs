/// LeetCode #2944 - Minimum Number of Coins for Fruits
fn minimum_coins(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let mut memo = vec![-1; n + 1];
    fn dfs(i: usize, prices: &[i32], memo: &mut [i32]) -> i32 {
        if i * 2 >= prices.len() {
            return prices[i - 1];
        }
        if memo[i] != -1 {
            return memo[i];
        }
        let mut best = i32::MAX;
        for j in (i + 1)..=(i * 2 + 1).min(prices.len()) {
            best = best.min(dfs(j, prices, memo));
        }
        memo[i] = prices[i - 1] + best;
        memo[i]
    }
    dfs(1, &prices, &mut memo)
}

fn main() {
    println!("{}", minimum_coins(vec![3, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::minimum_coins;

    #[test]
    fn example_one() {
        assert_eq!(minimum_coins(vec![3, 1, 2]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_coins(vec![1, 10, 1, 1]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_coins(vec![26, 18, 6, 12, 49, 7, 45, 45]), 39);
    }
}
