/// LeetCode #1407 - Top Travellers (SQL; Rust analogue)
use std::collections::HashMap;

fn top_travellers(users: Vec<(i32, String)>, rides: Vec<(i32, i32, i32)>) -> Vec<(String, i32)> {
    let mut dist: HashMap<i32, i32> = HashMap::new();
    for (_, uid, d) in rides {
        *dist.entry(uid).or_insert(0) += d;
    }
    let mut ans: Vec<(String, i32)> = users
        .into_iter()
        .map(|(id, name)| (name, *dist.get(&id).unwrap_or(&0)))
        .collect();
    ans.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    ans
}

fn main() {
    println!("{:?}", top_travellers(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::top_travellers;

    #[test]
    fn example() {
        let users = vec![
            (1, "Alice".into()),
            (2, "Bob".into()),
            (3, "Alex".into()),
            (4, "Donald".into()),
            (7, "Lee".into()),
            (13, "Jonathan".into()),
            (19, "Elvis".into()),
        ];
        let rides = vec![
            (1, 1, 120),
            (2, 2, 317),
            (3, 3, 222),
            (4, 7, 100),
            (5, 13, 312),
            (6, 19, 50),
            (7, 7, 120),
            (8, 19, 400),
            (9, 7, 230),
        ];
        assert_eq!(
            top_travellers(users, rides),
            vec![
                ("Elvis".into(), 450),
                ("Lee".into(), 450),
                ("Bob".into(), 317),
                ("Jonathan".into(), 312),
                ("Alex".into(), 222),
                ("Alice".into(), 120),
                ("Donald".into(), 0),
            ]
        );
    }
}
