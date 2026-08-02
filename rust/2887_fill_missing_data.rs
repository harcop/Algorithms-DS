/// LeetCode #2887 - Fill Missing Data (Pandas; Rust analogue)
fn fill_missing_values(
    products: Vec<(String, Option<i32>, i32)>,
) -> Vec<(String, i32, i32)> {
    products
        .into_iter()
        .map(|(name, quantity, price)| (name, quantity.unwrap_or(0), price))
        .collect()
}

fn main() {
    let products = vec![
        ("Wristwatch".into(), None, 135),
        ("GolfClubs".into(), Some(779), 9319),
    ];
    println!("{:?}", fill_missing_values(products));
}

#[cfg(test)]
mod tests {
    use super::fill_missing_values;

    #[test]
    fn example() {
        let products = vec![
            ("Wristwatch".into(), None, 135),
            ("WirelessEarbuds".into(), None, 821),
            ("GolfClubs".into(), Some(779), 9319),
            ("Printer".into(), Some(849), 3051),
        ];
        assert_eq!(
            fill_missing_values(products),
            vec![
                ("Wristwatch".into(), 0, 135),
                ("WirelessEarbuds".into(), 0, 821),
                ("GolfClubs".into(), 779, 9319),
                ("Printer".into(), 849, 3051),
            ]
        );
    }
}
