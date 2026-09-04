/// LeetCode #1571 - Warehouse Manager (SQL; Rust analogue)
use std::collections::HashMap;

fn warehouse_manager(
    warehouse: Vec<(String, i32, i32)>,
    products: Vec<(i32, String, i32, i32, i32)>,
) -> Vec<(String, i32)> {
    let vol: HashMap<i32, i32> = products
        .into_iter()
        .map(|(id, _, w, l, h)| (id, w * l * h))
        .collect();
    let mut ans: HashMap<String, i32> = HashMap::new();
    for (name, pid, units) in warehouse {
        *ans.entry(name).or_insert(0) += units * vol[&pid];
    }
    ans.into_iter().collect()
}

fn main() {
    println!("{:?}", warehouse_manager(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::warehouse_manager;

    #[test]
    fn example() {
        let warehouse = vec![
            ("LCHouse1".into(), 1, 1),
            ("LCHouse1".into(), 2, 10),
            ("LCHouse1".into(), 3, 5),
            ("LCHouse2".into(), 1, 2),
            ("LCHouse2".into(), 2, 2),
            ("LCHouse3".into(), 4, 1),
        ];
        let products = vec![
            (1, "LC-TV".into(), 5, 50, 40),
            (2, "LC-KeyChain".into(), 5, 5, 5),
            (3, "LC-Phone".into(), 2, 10, 10),
            (4, "LC-T-Shirt".into(), 4, 10, 20),
        ];
        let mut got = warehouse_manager(warehouse, products);
        got.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(
            got,
            vec![
                ("LCHouse1".into(), 12250),
                ("LCHouse2".into(), 20250),
                ("LCHouse3".into(), 800),
            ]
        );
    }
}
