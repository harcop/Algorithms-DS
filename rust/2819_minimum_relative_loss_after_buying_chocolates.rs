/// LeetCode #2819 - Minimum Relative Loss After Buying Chocolates
fn minimum_relative_loss(prices: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let mut prices = prices;
    prices.sort_unstable();
    let n = prices.len();
    let mut prefix = vec![0i64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + prices[i] as i64;
    }

    queries
        .into_iter()
        .map(|q| {
            let k = q[0] as i64;
            let m = q[1] as usize;
            let count_no_gt_k = prices.partition_point(|&p| p <= q[0]);
            let mut l = 0usize;
            let mut r = count_no_gt_k.min(m);
            while l < r {
                let mid = (l + r) / 2;
                let back = m - mid;
                if (prices[mid] as i64) < (2 * k - prices[n - back] as i64) {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }
            let front = l;
            let back = m - front;
            prefix[front] + 2 * k * back as i64 - (prefix[n] - prefix[n - back])
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        minimum_relative_loss(vec![1, 9, 22, 10, 19], vec![vec![18, 4], vec![5, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_relative_loss;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_relative_loss(vec![1, 9, 22, 10, 19], vec![vec![18, 4], vec![5, 2]]),
            vec![34, -21]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_relative_loss(vec![5, 6, 7], vec![vec![10, 1], vec![5, 3], vec![3, 3]]),
            vec![5, 12, 0]
        );
    }
}
