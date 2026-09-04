/// LeetCode #1070 - Product Sales Analysis III (SQL; Rust analogue)
use std::collections::HashMap;

fn product_sales_iii(sales: Vec<(i32, i32, i32, i32, i32)>) -> Vec<(i32, i32, i32, i32)> {
    let mut first: HashMap<i32, i32> = HashMap::new();
    for (_, pid, year, _, _) in &sales {
        first
            .entry(*pid)
            .and_modify(|y| *y = (*y).min(*year))
            .or_insert(*year);
    }
    let mut ans: Vec<(i32, i32, i32, i32)> = sales
        .into_iter()
        .filter(|(_, pid, year, _, _)| first.get(pid) == Some(year))
        .map(|(_, pid, year, q, p)| (pid, year, q, p))
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::product_sales_iii;

    #[test]
    fn example() {
        let sales = vec![
            (1, 100, 2008, 10, 5000),
            (2, 100, 2009, 12, 5000),
            (7, 200, 2011, 15, 9000),
        ];
        assert_eq!(
            product_sales_iii(sales),
            vec![(100, 2008, 10, 5000), (200, 2011, 15, 9000)]
        );
    }
}
