/// LeetCode #3573 - Best Time to Buy and Sell Stock V
fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
    let n = prices.len();
    let k = k as usize;
    let mut f = vec![vec![vec![0i64; 3]; k + 1]; n];
    for j in 1..=k {
        f[0][j][1] = -(prices[0] as i64);
        f[0][j][2] = prices[0] as i64;
    }
    for i in 1..n {
        for j in 1..=k {
            f[i][j][0] = f[i - 1][j][0]
                .max(f[i - 1][j][1] + prices[i] as i64)
                .max(f[i - 1][j][2] - prices[i] as i64);
            f[i][j][1] = f[i - 1][j][1].max(f[i - 1][j - 1][0] - prices[i] as i64);
            f[i][j][2] = f[i - 1][j][2].max(f[i - 1][j - 1][0] + prices[i] as i64);
        }
    }
    f[n - 1][k][0]
}

fn main() {
    println!("{}", maximum_profit(vec![1, 7, 9, 8, 2], 2));
}

#[cfg(test)]
mod tests {
    use super::maximum_profit;

    #[test]
    fn example1() {
        assert_eq!(maximum_profit(vec![1, 7, 9, 8, 2], 2), 14);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_profit(vec![12, 16, 19, 19, 8, 1, 19, 13, 9], 3), 36);
    }
}
