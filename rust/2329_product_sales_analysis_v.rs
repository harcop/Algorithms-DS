/// LeetCode #2329 - Product Sales Analysis V (SQL; Rust analogue)
use std::collections::HashMap;

fn product_sales_analysis_v(
    sales: Vec<(i32, i32, i32, i32)>,
    product: Vec<(i32, i32)>,
) -> Vec<(i32, i64)> {
    let price: HashMap<i32, i32> = product.into_iter().collect();
    let mut spend: HashMap<i32, i64> = HashMap::new();
    for (_sale_id, product_id, user_id, quantity) in sales {
        let p = *price.get(&product_id).unwrap_or(&0);
        *spend.entry(user_id).or_insert(0) += quantity as i64 * p as i64;
    }
    let mut ans: Vec<(i32, i64)> = spend.into_iter().collect();
    ans.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    ans
}

fn main() {
    println!("{:?}", product_sales_analysis_v(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::product_sales_analysis_v;

    #[test]
    fn example_one() {
        let sales = vec![
            (1, 1, 101, 10),
            (2, 2, 101, 1),
            (3, 3, 102, 3),
            (4, 3, 102, 2),
            (5, 2, 103, 3),
        ];
        let product = vec![(1, 10), (2, 25), (3, 15)];
        assert_eq!(
            product_sales_analysis_v(sales, product),
            vec![(101, 125), (102, 75), (103, 75)]
        );
    }
}
