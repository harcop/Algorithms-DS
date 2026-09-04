/// LeetCode #1069 - Product Sales Analysis II (SQL; Rust analogue)
use std::collections::HashMap;

fn product_sales_ii(sales: Vec<(i32, i32, i32, i32, i32)>) -> Vec<(i32, i32)> {
    let mut qty: HashMap<i32, i32> = HashMap::new();
    for (_, pid, _, q, _) in sales {
        *qty.entry(pid).or_insert(0) += q;
    }
    let mut ans: Vec<(i32, i32)> = qty.into_iter().collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::product_sales_ii;

    #[test]
    fn example() {
        let sales = vec![
            (1, 100, 2008, 10, 5000),
            (2, 100, 2009, 12, 5000),
            (7, 200, 2011, 15, 9000),
        ];
        assert_eq!(product_sales_ii(sales), vec![(100, 22), (200, 15)]);
    }
}
