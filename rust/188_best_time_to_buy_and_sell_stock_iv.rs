/// LeetCode #188 - Best Time to Buy and Sell Stock IV
fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    let n = prices.len();
    if n == 0 || k == 0 {
        return 0;
    }
    let k = k as usize;
    if k >= n / 2 {
        let mut p = 0i32;
        for i in 1..n {
            p += (prices[i] - prices[i - 1]).max(0);
        }
        return p;
    }
    let mut buy = vec![i32::MIN / 4; k + 1];
    let mut sell = vec![0; k + 1];
    for &price in &prices {
        for t in 1..=k {
            buy[t] = buy[t].max(sell[t - 1] - price);
            sell[t] = sell[t].max(buy[t] + price);
        }
    }
    sell[k]
}

fn main() {
    println!("{}", max_profit(2, vec![2, 4, 1]));
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn example_one() {
        assert_eq!(max_profit(2, vec![2, 4, 1]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
    }
}
