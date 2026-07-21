/// LeetCode #2548 - Maximum Price to Fill a Bag
fn max_price(mut items: Vec<Vec<i32>>, capacity: i32) -> f64 {
    let mut capacity = capacity as i64;
    let mut ans = 0.0;

    items.sort_by(|a, b| {
        let ra = a[0] as f64 / a[1] as f64;
        let rb = b[0] as f64 / b[1] as f64;
        rb.partial_cmp(&ra).unwrap()
    });

    for item in items {
        let price = item[0] as i64;
        let weight = item[1] as i64;
        if capacity <= weight {
            return ans + price as f64 * capacity as f64 / weight as f64;
        }
        ans += price as f64;
        capacity -= weight;
    }

    -1.0
}

fn main() {
    let items = vec![vec![50, 1], vec![10, 8]];
    println!("{}", max_price(items, 5));
}

#[cfg(test)]
mod tests {
    use super::max_price;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-5
    }

    #[test]
    fn example_one() {
        let items = vec![vec![50, 1], vec![10, 8]];
        assert!(approx_eq(max_price(items, 5), 55.0));
    }

    #[test]
    fn example_two() {
        let items = vec![vec![100, 30]];
        assert!(approx_eq(max_price(items, 50), -1.0));
    }
}
