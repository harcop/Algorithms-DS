/// LeetCode #1677 - Product's Price for Each Store (SQL; Rust analogue)
use std::collections::BTreeMap;

fn products_price(products: Vec<(i32, String, i32)>) -> Vec<(i32, Option<i32>, Option<i32>, Option<i32>)> {
    let mut by_id: BTreeMap<i32, (Option<i32>, Option<i32>, Option<i32>)> = BTreeMap::new();
    for (id, store, price) in products {
        let e = by_id.entry(id).or_insert((None, None, None));
        match store.as_str() {
            "store1" => e.0 = Some(price),
            "store2" => e.1 = Some(price),
            "store3" => e.2 = Some(price),
            _ => {}
        }
    }
    by_id
        .into_iter()
        .map(|(id, (a, b, c))| (id, a, b, c))
        .collect()
}

fn main() {
    println!("{:?}", products_price(vec![]));
}

#[cfg(test)]
mod tests {
    use super::products_price;

    #[test]
    fn example() {
        let products = vec![
            (0, "store1".into(), 95),
            (0, "store3".into(), 105),
            (0, "store2".into(), 100),
            (1, "store1".into(), 70),
            (1, "store3".into(), 80),
        ];
        assert_eq!(
            products_price(products),
            vec![
                (0, Some(95), Some(100), Some(105)),
                (1, Some(70), None, Some(80)),
            ]
        );
    }
}
