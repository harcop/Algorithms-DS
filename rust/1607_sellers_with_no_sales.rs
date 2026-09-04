/// LeetCode #1607 - Sellers With No Sales (SQL; Rust analogue)
use std::collections::HashSet;

fn sellers_with_no_sales(
    _customer: Vec<(i32, String)>,
    orders: Vec<(i32, String, i32, i32, i32)>,
    seller: Vec<(i32, String)>,
) -> Vec<String> {
    let sold_2020: HashSet<i32> = orders
        .into_iter()
        .filter(|(_, date, _, _, _)| date.starts_with("2020"))
        .map(|(_, _, _, _, sid)| sid)
        .collect();
    let mut ans: Vec<String> = seller
        .into_iter()
        .filter(|(id, _)| !sold_2020.contains(id))
        .map(|(_, name)| name)
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("{:?}", sellers_with_no_sales(vec![], vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::sellers_with_no_sales;

    #[test]
    fn example() {
        let customer = vec![
            (101, "Alice".into()),
            (102, "Bob".into()),
            (103, "Charlie".into()),
        ];
        let orders = vec![
            (1, "2020-03-01".into(), 1500, 101, 1),
            (2, "2020-05-25".into(), 2400, 102, 2),
            (3, "2019-05-25".into(), 800, 101, 3),
            (4, "2020-09-13".into(), 1000, 103, 2),
            (5, "2019-02-11".into(), 700, 101, 2),
        ];
        let seller = vec![
            (1, "Daniel".into()),
            (2, "Elizabeth".into()),
            (3, "Frank".into()),
        ];
        assert_eq!(
            sellers_with_no_sales(customer, orders, seller),
            vec!["Frank".to_string()]
        );
    }
}
