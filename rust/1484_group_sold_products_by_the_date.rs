/// LeetCode #1484 - Group Sold Products By The Date (SQL; Rust analogue)
use std::collections::{BTreeMap, BTreeSet};

fn group_sold_products(activities: Vec<(String, String)>) -> Vec<(String, i32, String)> {
    let mut by_date: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (date, product) in activities {
        by_date.entry(date).or_default().insert(product);
    }
    by_date
        .into_iter()
        .map(|(date, set)| {
            let n = set.len() as i32;
            let products = set.into_iter().collect::<Vec<_>>().join(",");
            (date, n, products)
        })
        .collect()
}

fn main() {
    println!("{:?}", group_sold_products(vec![]));
}

#[cfg(test)]
mod tests {
    use super::group_sold_products;

    #[test]
    fn example() {
        let activities = vec![
            ("2020-05-30".into(), "Headphone".into()),
            ("2020-06-01".into(), "Pencil".into()),
            ("2020-06-02".into(), "Mask".into()),
            ("2020-05-30".into(), "Basketball".into()),
            ("2020-06-01".into(), "Bible".into()),
            ("2020-06-02".into(), "Mask".into()),
            ("2020-05-30".into(), "T-Shirt".into()),
        ];
        assert_eq!(
            group_sold_products(activities),
            vec![
                ("2020-05-30".into(), 3, "Basketball,Headphone,T-Shirt".into()),
                ("2020-06-01".into(), 2, "Bible,Pencil".into()),
                ("2020-06-02".into(), 1, "Mask".into()),
            ]
        );
    }
}
