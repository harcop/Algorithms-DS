/// LeetCode #2324 - Product Sales Analysis IV (SQL; Rust analogue)
use std::collections::HashMap;

fn product_sales_analysis_iv(
    sales: Vec<(i32, i32, i32, i32)>,
    product: Vec<(i32, i32)>,
) -> Vec<(i32, i32)> {
    let price: HashMap<i32, i32> = product.into_iter().collect();
    let mut spend: HashMap<(i32, i32), i64> = HashMap::new();
    for (_sale_id, product_id, user_id, quantity) in sales {
        let p = *price.get(&product_id).unwrap_or(&0);
        *spend.entry((user_id, product_id)).or_insert(0) += quantity as i64 * p as i64;
    }
    let mut best: HashMap<i32, i64> = HashMap::new();
    for ((user_id, _), amount) in &spend {
        best.entry(*user_id)
            .and_modify(|m| *m = (*m).max(*amount))
            .or_insert(*amount);
    }
    let mut ans: Vec<(i32, i32)> = spend
        .into_iter()
        .filter(|((user_id, _), amount)| best.get(user_id) == Some(amount))
        .map(|((user_id, product_id), _)| (user_id, product_id))
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("{:?}", product_sales_analysis_iv(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::product_sales_analysis_iv;

    #[test]
    fn example_one() {
        let sales = vec![
            (1, 1, 101, 10),
            (2, 3, 101, 7),
            (3, 1, 102, 9),
            (4, 2, 102, 6),
            (5, 3, 102, 10),
            (6, 1, 102, 6),
        ];
        let product = vec![(1, 10), (2, 25), (3, 15)];
        assert_eq!(
            product_sales_analysis_iv(sales, product),
            vec![(101, 3), (102, 1), (102, 2), (102, 3)]
        );
    }
}
