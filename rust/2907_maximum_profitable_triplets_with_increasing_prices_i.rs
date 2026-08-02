/// LeetCode #2907 - Maximum Profitable Triplets With Increasing Prices I
fn max_profit(prices: Vec<i32>, profits: Vec<i32>) -> i32 {
    let n = prices.len();
    let mut ans = -1;
    for j in 0..n {
        let mut left = 0;
        let mut right = 0;
        for i in 0..j {
            if prices[i] < prices[j] {
                left = left.max(profits[i]);
            }
        }
        for k in j + 1..n {
            if prices[j] < prices[k] {
                right = right.max(profits[k]);
            }
        }
        if left > 0 && right > 0 {
            ans = ans.max(left + profits[j] + right);
        }
    }
    ans
}

fn main() {
    println!("{}", max_profit(vec![10, 2, 3, 4], vec![100, 2, 7, 10]));
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn example_one() {
        assert_eq!(max_profit(vec![10, 2, 3, 4], vec![100, 2, 7, 10]), 19);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5], vec![1, 5, 3, 4, 6]), 15);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_profit(vec![4, 3, 2, 1], vec![33, 20, 19, 87]), -1);
    }
}
