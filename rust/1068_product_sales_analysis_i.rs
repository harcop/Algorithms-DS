/// LeetCode #1068 - Product Sales Analysis I (SQL; Rust analogue)
use std::collections::HashMap;

fn product_sales_i(
    sales: Vec<(i32, i32, i32, i32, i32)>,
    product: Vec<(i32, String)>,
) -> Vec<(String, i32, i32)> {
    let names: HashMap<i32, String> = product.into_iter().collect();
    sales
        .into_iter()
        .map(|(_, pid, year, _, price)| (names[&pid].clone(), year, price))
        .collect()
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::product_sales_i;

    #[test]
    fn example() {
        let sales = vec![
            (1, 100, 2008, 10, 5000),
            (2, 100, 2009, 12, 5000),
            (7, 200, 2011, 15, 9000),
        ];
        let product = vec![
            (100, "Nokia".into()),
            (200, "Apple".into()),
            (300, "Samsung".into()),
        ];
        assert_eq!(
            product_sales_i(sales, product),
            vec![
                ("Nokia".into(), 2008, 5000),
                ("Nokia".into(), 2009, 5000),
                ("Apple".into(), 2011, 9000),
            ]
        );
    }
}
