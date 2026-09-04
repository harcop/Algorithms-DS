/// LeetCode #1795 - Rearrange Products Table (SQL; Rust analogue)
fn rearrange_products_table(
    products: Vec<(i32, Option<i32>, Option<i32>, Option<i32>)>,
) -> Vec<(i32, String, i32)> {
    let mut ans = Vec::new();
    for (product_id, store1, store2, store3) in products {
        if let Some(price) = store1 {
            ans.push((product_id, "store1".into(), price));
        }
        if let Some(price) = store2 {
            ans.push((product_id, "store2".into(), price));
        }
        if let Some(price) = store3 {
            ans.push((product_id, "store3".into(), price));
        }
    }
    ans
}

fn main() {
    let products = vec![(0, Some(95), Some(100), Some(105)), (1, Some(70), None, Some(80))];
    println!("{:?}", rearrange_products_table(products));
}

#[cfg(test)]
mod tests {
    use super::rearrange_products_table;

    #[test]
    fn example_one() {
        let products = vec![
            (0, Some(95), Some(100), Some(105)),
            (1, Some(70), None, Some(80)),
        ];
        assert_eq!(
            rearrange_products_table(products),
            vec![
                (0, "store1".into(), 95),
                (0, "store2".into(), 100),
                (0, "store3".into(), 105),
                (1, "store1".into(), 70),
                (1, "store3".into(), 80),
            ]
        );
    }
}
