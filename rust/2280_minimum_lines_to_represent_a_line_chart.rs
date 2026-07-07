/// LeetCode #2280 - Minimum Lines to Represent a Line Chart
fn minimum_lines(mut stock_prices: Vec<Vec<i32>>) -> i32 {
    stock_prices.sort_unstable_by_key(|p| p[0]);
    if stock_prices.len() <= 1 {
        return 0;
    }

    let mut ans = 0;
    for i in 2..stock_prices.len() {
        let a = get_slope(&stock_prices[i - 2], &stock_prices[i - 1]);
        let b = get_slope(&stock_prices[i - 1], &stock_prices[i]);
        if a != b {
            ans += 1;
        }
    }

    ans + 1
}

fn get_slope(p: &[i32], q: &[i32]) -> (i32, i32) {
    let dx = p[0] - q[0];
    let dy = p[1] - q[1];
    if dx == 0 {
        return (0, p[0]);
    }
    if dy == 0 {
        return (p[1], 0);
    }
    let d = gcd(dx, dy);
    (dx / d, dy / d)
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn main() {
    println!(
        "{}",
        minimum_lines(vec![vec![1, 7], vec![2, 6], vec![3, 5], vec![4, 4], vec![5, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_lines;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_lines(vec![vec![1, 7], vec![2, 6], vec![3, 5], vec![4, 4], vec![5, 4]]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_lines(vec![vec![3, 4], vec![1, 2], vec![7, 8], vec![2, 3]]), 1);
    }
}
