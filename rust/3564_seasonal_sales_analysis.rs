/// LeetCode #3564 - Seasonal Sales Analysis (SQL; Rust analogue)
use std::collections::HashMap;

fn season_of(month: u32) -> &'static str {
    match month {
        12 | 1 | 2 => "Winter",
        3 | 4 | 5 => "Spring",
        6 | 7 | 8 => "Summer",
        _ => "Fall",
    }
}

fn seasonal_sales_analysis(
    sales: Vec<(i32, i32, (u32, u32, u32), i32, f64)>,
    products: Vec<(i32, String, String)>,
) -> Vec<(String, String, i32, f64)> {
    let category: HashMap<i32, String> = products
        .into_iter()
        .map(|(id, _, cat)| (id, cat))
        .collect();
    let mut agg: HashMap<(String, String), (i32, f64)> = HashMap::new();
    for (_sale_id, product_id, (_y, m, _d), qty, price) in sales {
        let Some(cat) = category.get(&product_id) else { continue };
        let season = season_of(m).to_string();
        let e = agg.entry((season, cat.clone())).or_insert((0, 0.0));
        e.0 += qty;
        e.1 += qty as f64 * price;
    }
    let mut best: HashMap<String, (String, i32, f64)> = HashMap::new();
    for ((season, cat), (qty, rev)) in agg {
        let replace = match best.get(&season) {
            None => true,
            Some((c, q, r)) => {
                qty > *q || (qty == *q && (rev > *r || (rev == *r && cat < *c)))
            }
        };
        if replace {
            best.insert(season, (cat, qty, rev));
        }
    }
    let mut ans: Vec<(String, String, i32, f64)> = best
        .into_iter()
        .map(|(season, (cat, qty, rev))| (season, cat, qty, rev))
        .collect();
    ans.sort_by(|a, b| a.0.cmp(&b.0));
    ans
}

fn main() {
    println!("{:?}", seasonal_sales_analysis(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::seasonal_sales_analysis;

    #[test]
    fn example() {
        let sales = vec![
            (1, 1, (2023, 1, 15), 5, 10.0),
            (2, 2, (2023, 1, 20), 4, 15.0),
            (3, 3, (2023, 3, 10), 3, 18.0),
            (4, 4, (2023, 4, 5), 1, 20.0),
            (5, 1, (2023, 5, 20), 2, 10.0),
            (6, 2, (2023, 6, 12), 4, 15.0),
            (7, 5, (2023, 6, 15), 5, 12.0),
            (8, 3, (2023, 7, 24), 2, 18.0),
            (9, 4, (2023, 8, 1), 5, 20.0),
            (10, 5, (2023, 9, 3), 3, 12.0),
            (11, 1, (2023, 9, 25), 6, 10.0),
            (12, 2, (2023, 11, 10), 4, 15.0),
            (13, 3, (2023, 12, 5), 6, 18.0),
            (14, 4, (2023, 12, 22), 3, 20.0),
            (15, 5, (2024, 2, 14), 2, 12.0),
        ];
        let products = vec![
            (1, "Warm Jacket".into(), "Apparel".into()),
            (2, "Designer Jeans".into(), "Apparel".into()),
            (3, "Cutting Board".into(), "Kitchen".into()),
            (4, "Smart Speaker".into(), "Tech".into()),
            (5, "Yoga Mat".into(), "Fitness".into()),
        ];
        assert_eq!(
            seasonal_sales_analysis(sales, products),
            vec![
                ("Fall".into(), "Apparel".into(), 10, 120.0),
                ("Spring".into(), "Kitchen".into(), 3, 54.0),
                ("Summer".into(), "Tech".into(), 5, 100.0),
                ("Winter".into(), "Apparel".into(), 9, 110.0),
            ]
        );
    }
}
