/// LeetCode #2922 - Market Analysis III (SQL; Rust analogue)
fn market_analysis(
    users: Vec<(i32, String)>,
    orders: Vec<(i32, i32)>,
    items: Vec<(i32, String)>,
) -> Vec<(i32, i32)> {
    use std::collections::{HashMap, HashSet};

    let favorite: HashMap<_, _> = users.into_iter().collect();
    let brand: HashMap<_, _> = items.into_iter().collect();

    let mut distinct: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (item_id, seller_id) in orders {
        let fav = favorite.get(&seller_id).map(String::as_str).unwrap_or("");
        let item_brand = brand.get(&item_id).map(String::as_str).unwrap_or("");
        if item_brand != fav {
            distinct.entry(seller_id).or_default().insert(item_id);
        }
    }

    let counts: Vec<(i32, i32)> = distinct
        .into_iter()
        .map(|(seller, set)| (seller, set.len() as i32))
        .collect();
    let max_items = counts.iter().map(|(_, c)| *c).max().unwrap_or(0);
    let mut ans: Vec<_> = counts
        .into_iter()
        .filter(|(_, c)| *c == max_items && max_items > 0)
        .collect();
    ans.sort_unstable();
    ans
}

fn main() {
    let users = vec![
        (1, "Lenovo".into()),
        (2, "Samsung".into()),
        (3, "LG".into()),
    ];
    let orders = vec![(4, 2), (2, 3), (3, 3), (1, 2), (4, 2)];
    let items = vec![
        (1, "Samsung".into()),
        (2, "Lenovo".into()),
        (3, "LG".into()),
        (4, "HP".into()),
    ];
    println!("{:?}", market_analysis(users, orders, items));
}

#[cfg(test)]
mod tests {
    use super::market_analysis;

    #[test]
    fn example() {
        let users = vec![
            (1, "Lenovo".into()),
            (2, "Samsung".into()),
            (3, "LG".into()),
        ];
        let orders = vec![(4, 2), (2, 3), (3, 3), (1, 2), (4, 2)];
        let items = vec![
            (1, "Samsung".into()),
            (2, "Lenovo".into()),
            (3, "LG".into()),
            (4, "HP".into()),
        ];
        assert_eq!(
            market_analysis(users, orders, items),
            vec![(2, 1), (3, 1)]
        );
    }
}
