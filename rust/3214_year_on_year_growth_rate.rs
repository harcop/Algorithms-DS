/// LeetCode #3214 - Year on Year Growth Rate (SQL; Rust analogue)
/// transactions: (transaction_id, product_id, spend, year)
fn year_on_year_growth(
    transactions: Vec<(i32, i32, f64, i32)>,
) -> Vec<(i32, i32, f64, Option<f64>, Option<f64>)> {
    use std::collections::BTreeMap;
    let mut spend: BTreeMap<(i32, i32), f64> = BTreeMap::new();
    for (_, product_id, amount, year) in transactions {
        *spend.entry((product_id, year)).or_insert(0.0) += amount;
    }
    let mut by_product: BTreeMap<i32, Vec<(i32, f64)>> = BTreeMap::new();
    for ((product_id, year), amount) in spend {
        by_product.entry(product_id).or_default().push((year, amount));
    }
    let mut ans = Vec::new();
    for (product_id, mut years) in by_product {
        years.sort_by_key(|&(y, _)| y);
        let lookup: BTreeMap<i32, f64> = years.iter().copied().collect();
        for (year, curr) in years {
            let prev = lookup.get(&(year - 1)).copied();
            let yoy = prev.map(|p| ((curr - p) / p * 10000.0).round() / 100.0);
            ans.push((year, product_id, curr, prev, yoy));
        }
    }
    ans
}

fn main() {
    let tx = vec![
        (1341, 123424, 1500.60, 2019),
        (1423, 123424, 1000.20, 2020),
        (1623, 123424, 1246.44, 2021),
        (1322, 123424, 2145.32, 2022),
    ];
    println!("{:?}", year_on_year_growth(tx));
}

#[cfg(test)]
mod tests {
    use super::year_on_year_growth;

    #[test]
    fn example() {
        let tx = vec![
            (1341, 123424, 1500.60, 2019),
            (1423, 123424, 1000.20, 2020),
            (1623, 123424, 1246.44, 2021),
            (1322, 123424, 2145.32, 2022),
        ];
        let got = year_on_year_growth(tx);
        assert_eq!(got.len(), 4);
        assert_eq!(got[0], (2019, 123424, 1500.60, None, None));
        assert_eq!(got[1].0, 2020);
        assert_eq!(got[1].3, Some(1500.60));
        assert_eq!(got[1].4, Some(-33.35));
        assert_eq!(got[2].0, 2021);
        assert_eq!(got[2].4, Some(24.62));
        assert_eq!(got[3].0, 2022);
        assert_eq!(got[3].4, Some(72.12));
    }
}
