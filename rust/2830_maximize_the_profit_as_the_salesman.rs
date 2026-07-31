/// LeetCode #2830 - Maximize the Profit as the Salesman
fn maximize_the_profit(n: i32, offers: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut offers: Vec<(usize, usize, i32)> = offers
        .into_iter()
        .map(|o| (o[0] as usize, o[1] as usize, o[2]))
        .collect();
    offers.sort_by_key(|&(_, end, _)| end);

    let m = offers.len();
    let mut dp = vec![0i32; m + 1];
    for i in 1..=m {
        let (start, end, gold) = offers[i - 1];
        let mut lo = 0usize;
        let mut hi = i - 1;
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if offers[mid - 1].1 < start {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        let prev = if lo > 0 { dp[lo] } else { 0 };
        dp[i] = dp[i - 1].max(prev + gold);
    }
    dp[m]
}

fn main() {
    println!(
        "{}",
        maximize_the_profit(5, vec![vec![0, 0, 1], vec![0, 2, 2], vec![1, 3, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::maximize_the_profit;

    #[test]
    fn example_one() {
        assert_eq!(
            maximize_the_profit(5, vec![vec![0, 0, 1], vec![0, 2, 2], vec![1, 3, 2]]),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximize_the_profit(5, vec![vec![0, 0, 1], vec![0, 2, 10], vec![1, 3, 2]]),
            10
        );
    }
}
