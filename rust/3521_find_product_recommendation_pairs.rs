/// LeetCode #3521 - Find Product Recommendation Pairs (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn find_product_recommendation_pairs(
    purchases: Vec<(i32, i32, i32)>,
    info: Vec<(i32, String, f64)>,
) -> Vec<(i32, i32, String, String, i32)> {
    let category: HashMap<i32, String> = info.into_iter().map(|(id, cat, _)| (id, cat)).collect();
    let mut by_user: HashMap<i32, Vec<i32>> = HashMap::new();
    for (user_id, product_id, _) in purchases {
        by_user.entry(user_id).or_default().push(product_id);
    }
    let mut pair_count: HashMap<(i32, i32), HashSet<i32>> = HashMap::new();
    for (user_id, mut products) in by_user {
        products.sort_unstable();
        products.dedup();
        for i in 0..products.len() {
            for j in i + 1..products.len() {
                pair_count
                    .entry((products[i], products[j]))
                    .or_default()
                    .insert(user_id);
            }
        }
    }
    let mut ans: Vec<(i32, i32, String, String, i32)> = pair_count
        .into_iter()
        .filter(|(_, users)| users.len() >= 3)
        .map(|((a, b), users)| {
            (
                a,
                b,
                category.get(&a).cloned().unwrap_or_default(),
                category.get(&b).cloned().unwrap_or_default(),
                users.len() as i32,
            )
        })
        .collect();
    ans.sort_by(|x, y| y.4.cmp(&x.4).then(x.0.cmp(&y.0)).then(x.1.cmp(&y.1)));
    ans
}

fn main() {
    let purchases = vec![(1, 101, 2), (1, 102, 1)];
    let info = vec![(101, "Electronics".into(), 100.0), (102, "Books".into(), 20.0)];
    println!("{:?}", find_product_recommendation_pairs(purchases, info));
}

#[cfg(test)]
mod tests {
    use super::find_product_recommendation_pairs;

    #[test]
    fn example() {
        let purchases = vec![
            (1, 101, 2),
            (1, 102, 1),
            (1, 103, 3),
            (2, 101, 1),
            (2, 102, 5),
            (2, 104, 1),
            (3, 101, 2),
            (3, 103, 1),
            (3, 105, 4),
            (4, 101, 1),
            (4, 102, 1),
            (4, 103, 2),
            (4, 104, 3),
            (5, 102, 2),
            (5, 104, 1),
        ];
        let info = vec![
            (101, "Electronics".into(), 100.0),
            (102, "Books".into(), 20.0),
            (103, "Clothing".into(), 35.0),
            (104, "Kitchen".into(), 50.0),
            (105, "Sports".into(), 75.0),
        ];
        assert_eq!(
            find_product_recommendation_pairs(purchases, info),
            vec![
                (101, 102, "Electronics".into(), "Books".into(), 3),
                (101, 103, "Electronics".into(), "Clothing".into(), 3),
                (102, 104, "Books".into(), "Kitchen".into(), 3),
            ]
        );
    }
}
