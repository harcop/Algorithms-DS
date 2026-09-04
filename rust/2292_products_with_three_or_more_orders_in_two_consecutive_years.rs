/// LeetCode #2292 - Products With Three or More Orders in Two Consecutive Years (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn products_three_orders_consecutive_years(orders: Vec<(i32, i32, i32, String)>) -> Vec<i32> {
    let mut counts: HashMap<(i32, i32), i32> = HashMap::new();
    for (_oid, product_id, _qty, date) in orders {
        let year: i32 = date[..4].parse().unwrap();
        *counts.entry((product_id, year)).or_insert(0) += 1;
    }
    let mut years_ok: HashMap<i32, HashSet<i32>> = HashMap::new();
    for ((product_id, year), c) in counts {
        if c >= 3 {
            years_ok.entry(product_id).or_default().insert(year);
        }
    }
    let mut ans: Vec<i32> = years_ok
        .into_iter()
        .filter(|(_, ys)| ys.iter().any(|y| ys.contains(&(y + 1))))
        .map(|(pid, _)| pid)
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("{:?}", products_three_orders_consecutive_years(vec![]));
}

#[cfg(test)]
mod tests {
    use super::products_three_orders_consecutive_years;

    #[test]
    fn example_one() {
        let orders = vec![
            (1, 1, 7, "2020-03-16".into()),
            (2, 1, 4, "2020-12-02".into()),
            (3, 1, 7, "2020-05-10".into()),
            (4, 1, 6, "2021-12-23".into()),
            (5, 1, 5, "2021-05-21".into()),
            (6, 1, 6, "2021-10-11".into()),
            (7, 2, 6, "2022-10-11".into()),
        ];
        assert_eq!(products_three_orders_consecutive_years(orders), vec![1]);
    }
}
