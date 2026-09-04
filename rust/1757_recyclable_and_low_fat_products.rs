/// LeetCode #1757 - Recyclable and Low Fat Products (SQL; Rust analogue)
fn recyclable_low_fat(products: Vec<(i32, String, String)>) -> Vec<i32> {
    products
        .into_iter()
        .filter(|(_, low, rec)| low == "Y" && rec == "Y")
        .map(|(id, _, _)| id)
        .collect()
}

fn main() {
    println!("{:?}", recyclable_low_fat(vec![]));
}

#[cfg(test)]
mod tests {
    use super::recyclable_low_fat;

    #[test]
    fn example() {
        let products = vec![
            (0, "Y".into(), "N".into()),
            (1, "Y".into(), "Y".into()),
            (2, "N".into(), "Y".into()),
            (3, "Y".into(), "Y".into()),
            (4, "N".into(), "N".into()),
        ];
        assert_eq!(recyclable_low_fat(products), vec![1, 3]);
    }
}
