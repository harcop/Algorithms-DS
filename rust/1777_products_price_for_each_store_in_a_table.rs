/// LeetCode #1777 - Product's Price for Each Store (SQL; Rust analogue)
use std::collections::BTreeMap;

fn products_price_for_each_store(
    products: Vec<(i32, String, i32)>,
) -> Vec<(i32, Option<i32>, Option<i32>, Option<i32>)> {
    let mut map: BTreeMap<i32, (Option<i32>, Option<i32>, Option<i32>)> = BTreeMap::new();
    for (product_id, store, price) in products {
        let entry = map.entry(product_id).or_insert((None, None, None));
        match store.as_str() {
            "store1" => entry.0 = Some(price),
            "store2" => entry.1 = Some(price),
            "store3" => entry.2 = Some(price),
            _ => {}
        }
    }
    map.into_iter()
        .map(|(id, (s1, s2, s3))| (id, s1, s2, s3))
        .collect()
}

fn main() {
    let products = vec![
        (0, "store1".into(), 95),
        (0, "store3".into(), 105),
        (0, "store2".into(), 100),
        (1, "store1".into(), 70),
        (1, "store3".into(), 80),
    ];
    println!("{:?}", products_price_for_each_store(products));
}

#[cfg(test)]
mod tests {
    use super::products_price_for_each_store;

    #[test]
    fn example_one() {
        let products = vec![
            (0, "store1".into(), 95),
            (0, "store3".into(), 105),
            (0, "store2".into(), 100),
            (1, "store1".into(), 70),
            (1, "store3".into(), 80),
        ];
        assert_eq!(
            products_price_for_each_store(products),
            vec![
                (0, Some(95), Some(100), Some(105)),
                (1, Some(70), None, Some(80)),
            ]
        );
    }
}
