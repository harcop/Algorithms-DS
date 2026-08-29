/// LeetCode #3465 - Find Products with Valid Serial Numbers (SQL; Rust analogue)
fn has_valid_serial(description: &str) -> bool {
    let b = description.as_bytes();
    let n = b.len();
    let is_word = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    let mut i = 0;
    while i + 11 <= n {
        if b[i] == b'S' && b[i + 1] == b'N' {
            let before_ok = i == 0 || !is_word(b[i - 1]);
            let digits1 = (2..6).all(|k| b[i + k].is_ascii_digit());
            let hyphen = b[i + 6] == b'-';
            let digits2 = (7..11).all(|k| b[i + k].is_ascii_digit());
            let after_ok = i + 11 == n || !is_word(b[i + 11]);
            if before_ok && digits1 && hyphen && digits2 && after_ok {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn find_valid_serial_products(
    mut products: Vec<(i32, String, String)>,
) -> Vec<(i32, String, String)> {
    products.retain(|(_, _, desc)| has_valid_serial(desc));
    products.sort_by_key(|(id, _, _)| *id);
    products
}

fn main() {
    let products = vec![
        (
            1,
            "Widget A".into(),
            "This is a sample product with SN1234-5678".into(),
        ),
        (
            2,
            "Widget B".into(),
            "A product with serial SN9876-1234 in the description".into(),
        ),
        (
            3,
            "Widget C".into(),
            "Product SN1234-56789 is available now".into(),
        ),
        (4, "Widget D".into(), "No serial number here".into()),
        (
            5,
            "Widget E".into(),
            "Check out SN4321-8765 in this description".into(),
        ),
    ];
    println!("{:?}", find_valid_serial_products(products));
}

#[cfg(test)]
mod tests {
    use super::find_valid_serial_products;

    #[test]
    fn example() {
        let products = vec![
            (
                1,
                "Widget A".into(),
                "This is a sample product with SN1234-5678".into(),
            ),
            (
                2,
                "Widget B".into(),
                "A product with serial SN9876-1234 in the description".into(),
            ),
            (
                3,
                "Widget C".into(),
                "Product SN1234-56789 is available now".into(),
            ),
            (4, "Widget D".into(), "No serial number here".into()),
            (
                5,
                "Widget E".into(),
                "Check out SN4321-8765 in this description".into(),
            ),
        ];
        assert_eq!(
            find_valid_serial_products(products),
            vec![
                (
                    1,
                    "Widget A".into(),
                    "This is a sample product with SN1234-5678".into(),
                ),
                (
                    2,
                    "Widget B".into(),
                    "A product with serial SN9876-1234 in the description".into(),
                ),
                (
                    5,
                    "Widget E".into(),
                    "Check out SN4321-8765 in this description".into(),
                ),
            ]
        );
    }
}
