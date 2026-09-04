/// LeetCode #1445 - Apples & Oranges (SQL; Rust analogue)
use std::collections::BTreeMap;

fn apples_oranges(sales: Vec<(String, String, i32)>) -> Vec<(String, i32)> {
    let mut by_date: BTreeMap<String, (i32, i32)> = BTreeMap::new();
    for (date, fruit, num) in sales {
        let e = by_date.entry(date).or_insert((0, 0));
        if fruit == "apples" {
            e.0 += num;
        } else {
            e.1 += num;
        }
    }
    by_date
        .into_iter()
        .map(|(d, (a, o))| (d, a - o))
        .collect()
}

fn main() {
    println!("{:?}", apples_oranges(vec![]));
}

#[cfg(test)]
mod tests {
    use super::apples_oranges;

    #[test]
    fn example() {
        let sales = vec![
            ("2020-05-01".into(), "apples".into(), 10),
            ("2020-05-01".into(), "oranges".into(), 8),
            ("2020-05-02".into(), "apples".into(), 15),
            ("2020-05-02".into(), "oranges".into(), 15),
            ("2020-05-03".into(), "apples".into(), 20),
            ("2020-05-03".into(), "oranges".into(), 0),
            ("2020-05-04".into(), "apples".into(), 15),
            ("2020-05-04".into(), "oranges".into(), 16),
        ];
        assert_eq!(
            apples_oranges(sales),
            vec![
                ("2020-05-01".into(), 2),
                ("2020-05-02".into(), 0),
                ("2020-05-03".into(), 20),
                ("2020-05-04".into(), -1),
            ]
        );
    }
}
