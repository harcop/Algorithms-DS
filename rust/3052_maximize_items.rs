/// LeetCode #3052 - Maximize Items (SQL; Rust analogue)
fn maximize_items(inventory: Vec<(i32, String, String, f64)>) -> Vec<(String, i64)> {
    const WAREHOUSE: f64 = 500_000.0;

    let mut prime_sum = 0.0;
    let mut prime_cnt = 0i64;
    let mut not_sum = 0.0;
    let mut not_cnt = 0i64;

    for (_, _, category, sqft) in &inventory {
        if category == "prime_eligible" {
            prime_sum += sqft;
            prime_cnt += 1;
        } else if category == "not_prime" {
            not_sum += sqft;
            not_cnt += 1;
        }
    }

    let (prime_items, rem) = if prime_sum > 0.0 {
        let sets = (WAREHOUSE / prime_sum).floor() as i64;
        let used = sets as f64 * prime_sum;
        (sets * prime_cnt, WAREHOUSE - used)
    } else {
        (0, WAREHOUSE)
    };

    let not_items = if not_sum > 0.0 {
        (rem / not_sum).floor() as i64 * not_cnt
    } else {
        0
    };

    vec![
        ("prime_eligible".to_string(), prime_items),
        ("not_prime".to_string(), not_items),
    ]
}

fn example_inventory() -> Vec<(i32, String, String, f64)> {
    vec![
        (1, "Prime Storage".into(), "prime_eligible".into(), 92.54),
        (2, "Prime Storage".into(), "prime_eligible".into(), 92.54),
        (3, "Prime Storage".into(), "prime_eligible".into(), 92.53),
        (4, "Prime Storage".into(), "prime_eligible".into(), 92.53),
        (5, "Prime Storage".into(), "prime_eligible".into(), 92.53),
        (6, "Prime Storage".into(), "prime_eligible".into(), 92.53),
        (7, "Not Prime Storage".into(), "not_prime".into(), 32.13),
        (8, "Not Prime Storage".into(), "not_prime".into(), 32.12),
        (9, "Not Prime Storage".into(), "not_prime".into(), 32.12),
        (10, "Not Prime Storage".into(), "not_prime".into(), 32.13),
    ]
}

fn main() {
    let inventory = example_inventory();
    println!("{:?}", maximize_items(inventory));
}

#[cfg(test)]
mod tests {
    use super::{example_inventory, maximize_items};

    #[test]
    fn example() {
        let inventory = example_inventory();
        assert_eq!(
            maximize_items(inventory),
            vec![
                ("prime_eligible".into(), 5400),
                ("not_prime".into(), 8),
            ]
        );
    }
}
