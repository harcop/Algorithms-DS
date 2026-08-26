/// LeetCode #3415 - Find Products with Three Consecutive Digits (SQL; Rust analogue)
fn has_exactly_three_consecutive_digits(name: &str) -> bool {
    let b = name.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i].is_ascii_digit() {
            let start = i;
            while i < b.len() && b[i].is_ascii_digit() {
                i += 1;
            }
            if i - start == 3 {
                return true;
            }
        } else {
            i += 1;
        }
    }
    false
}

fn find_products(mut products: Vec<(i32, String)>) -> Vec<(i32, String)> {
    products.retain(|(_, name)| has_exactly_three_consecutive_digits(name));
    products.sort_by_key(|(id, _)| *id);
    products
}

fn main() {
    let products = vec![
        (1, "ABC123XYZ".into()),
        (2, "A12B34C".into()),
        (3, "Product56789".into()),
        (4, "NoDigitsHere".into()),
        (5, "789Product".into()),
        (6, "Item003Description".into()),
        (7, "Product12X34".into()),
    ];
    println!("{:?}", find_products(products));
}

#[cfg(test)]
mod tests {
    use super::find_products;

    #[test]
    fn example() {
        let products = vec![
            (1, "ABC123XYZ".into()),
            (2, "A12B34C".into()),
            (3, "Product56789".into()),
            (4, "NoDigitsHere".into()),
            (5, "789Product".into()),
            (6, "Item003Description".into()),
            (7, "Product12X34".into()),
        ];
        assert_eq!(
            find_products(products),
            vec![
                (1, "ABC123XYZ".into()),
                (5, "789Product".into()),
                (6, "Item003Description".into()),
            ]
        );
    }
}
