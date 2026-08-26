/// LeetCode #3413 - Maximum Coins From K Consecutive Bags
fn maximum_coins(mut coins: Vec<Vec<i32>>, k: i32) -> i64 {
    fn max_amount(coins: &mut [Vec<i32>], k: i64) -> i64 {
        coins.sort_unstable();
        let mut result = 0i64;
        let mut curr = 0i64;
        let mut left = 0usize;
        for right in 0..coins.len() {
            let (l, r, c) = (coins[right][0] as i64, coins[right][1] as i64, coins[right][2] as i64);
            curr += (r - l + 1) * c;
            while coins[right][1] as i64 - coins[left][1] as i64 + 1 > k {
                let (ll, rr, cc) = (
                    coins[left][0] as i64,
                    coins[left][1] as i64,
                    coins[left][2] as i64,
                );
                curr -= (rr - ll + 1) * cc;
                left += 1;
            }
            let extra = (coins[right][1] as i64 - coins[left][0] as i64 + 1 - k).max(0)
                * coins[left][2] as i64;
            result = result.max(curr - extra);
        }
        result
    }
    let k = k as i64;
    let a = max_amount(&mut coins, k);
    for coin in &mut coins {
        let l = coin[0];
        let r = coin[1];
        coin[0] = -r;
        coin[1] = -l;
    }
    a.max(max_amount(&mut coins, k))
}

fn main() {
    println!(
        "{}",
        maximum_coins(vec![vec![8, 10, 1], vec![1, 3, 2], vec![5, 6, 4]], 4)
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_coins;

    #[test]
    fn example1() {
        assert_eq!(
            maximum_coins(vec![vec![8, 10, 1], vec![1, 3, 2], vec![5, 6, 4]], 4),
            10
        );
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_coins(vec![vec![1, 10, 3]], 2), 6);
    }
}
