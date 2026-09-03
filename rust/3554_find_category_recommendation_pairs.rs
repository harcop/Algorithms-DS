/// LeetCode #3554 - Find Category Recommendation Pairs (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn find_category_recommendation_pairs(
    purchases: Vec<(i32, i32, i32)>,
    info: Vec<(i32, String, f64)>,
) -> Vec<(String, String, i32)> {
    let category: HashMap<i32, String> = info.into_iter().map(|(id, cat, _)| (id, cat)).collect();
    let mut by_user: HashMap<i32, HashSet<String>> = HashMap::new();
    for (user_id, product_id, _) in purchases {
        if let Some(cat) = category.get(&product_id) {
            by_user.entry(user_id).or_default().insert(cat.clone());
        }
    }
    let mut pair_count: HashMap<(String, String), HashSet<i32>> = HashMap::new();
    for (user_id, cats) in by_user {
        let mut cats: Vec<String> = cats.into_iter().collect();
        cats.sort();
        for i in 0..cats.len() {
            for j in i + 1..cats.len() {
                pair_count
                    .entry((cats[i].clone(), cats[j].clone()))
                    .or_default()
                    .insert(user_id);
            }
        }
    }
    let mut ans: Vec<(String, String, i32)> = pair_count
        .into_iter()
        .filter(|(_, users)| users.len() >= 3)
        .map(|((a, b), users)| (a, b, users.len() as i32))
        .collect();
    ans.sort_by(|x, y| y.2.cmp(&x.2).then(x.0.cmp(&y.0)).then(x.1.cmp(&y.1)));
    ans
}

fn main() {
    println!("{:?}", find_category_recommendation_pairs(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_category_recommendation_pairs;

    #[test]
    fn example() {
        let purchases = vec![
            (1, 101, 2),
            (1, 102, 1),
            (1, 201, 3),
            (1, 301, 1),
            (2, 101, 1),
            (2, 102, 2),
            (2, 103, 1),
            (2, 201, 5),
            (3, 101, 2),
            (3, 103, 1),
            (3, 301, 4),
            (3, 401, 2),
            (4, 101, 1),
            (4, 201, 3),
            (4, 301, 1),
            (4, 401, 2),
            (5, 102, 2),
            (5, 103, 1),
            (5, 201, 2),
            (5, 202, 3),
        ];
        let info = vec![
            (101, "Electronics".into(), 100.0),
            (102, "Books".into(), 20.0),
            (103, "Books".into(), 35.0),
            (201, "Clothing".into(), 45.0),
            (202, "Clothing".into(), 60.0),
            (301, "Sports".into(), 75.0),
            (401, "Kitchen".into(), 50.0),
        ];
        assert_eq!(
            find_category_recommendation_pairs(purchases, info),
            vec![
                ("Books".into(), "Clothing".into(), 3),
                ("Books".into(), "Electronics".into(), 3),
                ("Clothing".into(), "Electronics".into(), 3),
                ("Electronics".into(), "Sports".into(), 3),
            ]
        );
    }
}
