/// LeetCode #3652 - Best Time to Buy and Sell Stock using Strategy
fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
    let n = prices.len();
    let k = k as usize;
    let mut s = vec![0i64; n + 1];
    let mut t = vec![0i64; n + 1];
    for i in 1..=n {
        let a = prices[i - 1] as i64;
        let b = strategy[i - 1] as i64;
        s[i] = s[i - 1] + a * b;
        t[i] = t[i - 1] + a;
    }
    let mut ans = s[n];
    for i in k..=n {
        let cur = s[n] - (s[i] - s[i - k]) + (t[i] - t[i - k / 2]);
        ans = ans.max(cur);
    }
    ans
}

fn main() {
    println!("{}", max_profit(vec![4, 2, 8], vec![-1, 0, 1], 2));
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn example1() {
        assert_eq!(max_profit(vec![4, 2, 8], vec![-1, 0, 1], 2), 10);
    }

    #[test]
    fn example2() {
        assert_eq!(max_profit(vec![5, 4, 3], vec![1, 1, 0], 2), 9);
    }
}
