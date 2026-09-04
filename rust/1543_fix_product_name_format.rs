/// LeetCode #1543 - Fix Product Name Format (SQL; Rust analogue)
use std::collections::BTreeMap;

fn fix_product_name(sales: Vec<(i32, String, String)>) -> Vec<(String, String, i32)> {
    let mut cnt: BTreeMap<(String, String), i32> = BTreeMap::new();
    for (_, name, date) in sales {
        let product = name.trim().to_lowercase();
        let month = date[..7].to_string();
        *cnt.entry((product, month)).or_insert(0) += 1;
    }
    cnt.into_iter()
        .map(|((name, month), total)| (name, month, total))
        .collect()
}

fn main() {
    println!("{:?}", fix_product_name(vec![]));
}

#[cfg(test)]
mod tests {
    use super::fix_product_name;

    #[test]
    fn example() {
        let sales = vec![
            (1, "LCPHONE".into(), "2000-01-16".into()),
            (2, "LCPhone".into(), "2000-01-17".into()),
            (3, "LcPhOnE".into(), "2000-02-18".into()),
            (4, "LCKeyCHAiN".into(), "2000-02-19".into()),
            (5, "LCKeyChain".into(), "2000-02-28".into()),
            (6, "Matryoshka".into(), "2000-03-31".into()),
        ];
        assert_eq!(
            fix_product_name(sales),
            vec![
                ("lckeychain".into(), "2000-02".into(), 2),
                ("lcphone".into(), "2000-01".into(), 2),
                ("lcphone".into(), "2000-02".into(), 1),
                ("matryoshka".into(), "2000-03".into(), 1),
            ]
        );
    }
}
