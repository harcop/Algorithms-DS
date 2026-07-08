/// LeetCode #2291 - Maximum Profit From Trading Stocks
fn maximum_profit(present: Vec<i32>, future: Vec<i32>, budget: i32) -> i32 {
    let b = budget as usize;
    let mut dp = vec![0i32; b + 1];
    for (p, f) in present.into_iter().zip(future.into_iter()) {
        let cost = p as usize;
        let gain = f - p;
        if gain <= 0 || cost > b {
            continue;
        }
        for money in (cost..=b).rev() {
            dp[money] = dp[money].max(dp[money - cost] + gain);
        }
    }
    dp[b]
}

fn main() {
    println!(
        "{}",
        maximum_profit(vec![1, 2, 3], vec![2, 3, 4], 4)
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_profit;

    #[test]
    fn basic() {
        // Buy stocks 0 and 2: cost 1+3=4, profit (2-1)+(4-3)=2
        assert_eq!(maximum_profit(vec![1, 2, 3], vec![2, 3, 4], 4), 2);
    }

    #[test]
    fn skip_non_profitable() {
        assert_eq!(maximum_profit(vec![5, 4], vec![4, 4], 10), 0);
    }
}

