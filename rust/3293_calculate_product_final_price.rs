/// LeetCode #3293 - Calculate Product Final Price (SQL; Rust analogue)
use std::collections::HashMap;

fn calculate_final_prices(
    products: Vec<(i32, String, i32)>,
    discounts: Vec<(String, i32)>,
) -> Vec<(i32, i32, String)> {
    let disc: HashMap<String, i32> = discounts.into_iter().collect();
    let mut ans: Vec<_> = products
        .into_iter()
        .map(|(id, cat, price)| {
            let d = disc.get(&cat).copied().unwrap_or(0);
            (id, price * (100 - d) / 100, cat)
        })
        .collect();
    ans.sort_by_key(|x| x.0);
    ans
}

fn main() {
    let products = vec![
        (1, "Electronics".into(), 1000),
        (2, "Clothing".into(), 50),
    ];
    let discounts = vec![("Electronics".into(), 10), ("Clothing".into(), 20)];
    println!("{:?}", calculate_final_prices(products, discounts));
}

#[cfg(test)]
mod tests {
    use super::calculate_final_prices;

    #[test]
    fn example() {
        let products = vec![
            (1, "Electronics".into(), 1000),
            (2, "Clothing".into(), 50),
            (3, "Electronics".into(), 1200),
            (4, "Home".into(), 500),
        ];
        let discounts = vec![("Electronics".into(), 10), ("Clothing".into(), 20)];
        assert_eq!(
            calculate_final_prices(products, discounts),
            vec![
                (1, 900, "Electronics".into()),
                (2, 40, "Clothing".into()),
                (3, 1080, "Electronics".into()),
                (4, 500, "Home".into()),
            ]
        );
    }
}
