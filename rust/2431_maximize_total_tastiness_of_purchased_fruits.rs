/// LeetCode #2431 - Maximize Total Tastiness of Purchased Fruits
fn max_tastiness(price: Vec<i32>, tastiness: Vec<i32>, max_amount: i32, max_coupons: i32) -> i32 {
    let budget = max_amount as usize;
    let coupons = max_coupons as usize;
    let mut dp = vec![vec![-1; coupons + 1]; budget + 1];
    dp[0][0] = 0;

    for (price, taste) in price.into_iter().zip(tastiness) {
        let mut next = dp.clone();
        for spent in 0..=budget {
            for used in 0..=coupons {
                if dp[spent][used] < 0 {
                    continue;
                }

                let full_price = price as usize;
                if spent + full_price <= budget {
                    next[spent + full_price][used] =
                        next[spent + full_price][used].max(dp[spent][used] + taste);
                }

                let discounted_price = (price / 2) as usize;
                if used < coupons && spent + discounted_price <= budget {
                    next[spent + discounted_price][used + 1] =
                        next[spent + discounted_price][used + 1].max(dp[spent][used] + taste);
                }
            }
        }
        dp = next;
    }

    dp.into_iter().flatten().max().unwrap_or(0)
}

fn main() {
    println!("{}", max_tastiness(vec![10, 20, 20], vec![5, 8, 8], 20, 1));
}

#[cfg(test)]
mod tests {
    use super::max_tastiness;

    #[test]
    fn uses_one_coupon() {
        assert_eq!(max_tastiness(vec![10, 20, 20], vec![5, 8, 8], 20, 1), 13);
    }

    #[test]
    fn skips_expensive_fruit() {
        assert_eq!(max_tastiness(vec![30, 10], vec![20, 5], 10, 0), 5);
    }
}
