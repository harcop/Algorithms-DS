/// LeetCode #1159 - Market Analysis II (SQL; Rust analogue)
use std::collections::HashMap;

fn market_analysis_ii(
    users: Vec<(i32, String, String)>,
    mut orders: Vec<(i32, String, i32, i32, i32)>,
    items: Vec<(i32, String)>,
) -> Vec<(i32, String)> {
    let brand: HashMap<i32, String> = items.into_iter().collect();
    orders.sort_by(|a, b| a.4.cmp(&b.4).then(a.1.cmp(&b.1)));
    let mut by_seller: HashMap<i32, Vec<i32>> = HashMap::new();
    for (_, _, item, _, seller) in orders {
        by_seller.entry(seller).or_default().push(item);
    }
    let mut ans = Vec::new();
    for (uid, _, fav) in users {
        let yes = by_seller
            .get(&uid)
            .and_then(|v| v.get(1))
            .and_then(|item| brand.get(item))
            .map(|b| b == &fav)
            .unwrap_or(false);
        ans.push((uid, if yes { "yes" } else { "no" }.to_string()));
    }
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::market_analysis_ii;

    #[test]
    fn example() {
        let users = vec![
            (1, "2019-01-01".into(), "Lenovo".into()),
            (2, "2019-02-09".into(), "Samsung".into()),
            (3, "2019-01-19".into(), "LG".into()),
            (4, "2019-05-21".into(), "HP".into()),
        ];
        let orders = vec![
            (1, "2019-08-01".into(), 4, 1, 2),
            (2, "2019-08-02".into(), 2, 1, 3),
            (3, "2019-08-03".into(), 3, 2, 3),
            (4, "2019-08-04".into(), 1, 4, 2),
            (5, "2019-08-04".into(), 1, 3, 4),
            (6, "2019-08-05".into(), 2, 2, 4),
        ];
        let items = vec![
            (1, "Samsung".into()),
            (2, "Lenovo".into()),
            (3, "LG".into()),
            (4, "HP".into()),
        ];
        assert_eq!(
            market_analysis_ii(users, orders, items),
            vec![
                (1, "no".into()),
                (2, "yes".into()),
                (3, "yes".into()),
                (4, "no".into()),
            ]
        );
    }
}
