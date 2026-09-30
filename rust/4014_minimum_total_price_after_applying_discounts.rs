/// LeetCode #4014 - Minimum Total Price After Applying Discounts
fn min_price(mut prices: Vec<i32>, mut discounts: Vec<i32>) -> f64 {
    prices.sort_unstable();
    discounts.sort_unstable();
    let mut i = prices.len() as i32 - 1;
    let mut j = discounts.len() as i32 - 1;
    let mut ans = 0.0;
    while i >= 0 && j >= 0 {
        ans += prices[i as usize] as f64 * (100 - discounts[j as usize]) as f64 / 100.0;
        i -= 1;
        j -= 1;
    }
    while i >= 0 {
        ans += prices[i as usize] as f64;
        i -= 1;
    }
    ans
}

fn main() {
    println!("{}", min_price(vec![10, 30, 21], vec![50, 60]));
}

#[cfg(test)]
mod tests {
    use super::min_price;

    #[test]
    fn example1() {
        let ans = min_price(vec![10, 30, 21], vec![50, 60]);
        assert!((ans - 32.5).abs() < 1e-5);
    }

    #[test]
    fn example2() {
        let ans = min_price(vec![100, 70], vec![10, 40, 50]);
        assert!((ans - 92.0).abs() < 1e-5);
    }

    #[test]
    fn example3() {
        let ans = min_price(vec![7, 3, 9], vec![100, 100]);
        assert!((ans - 3.0).abs() < 1e-5);
    }
}
