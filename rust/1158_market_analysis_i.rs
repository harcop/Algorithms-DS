/// LeetCode #1158 - Market Analysis I (SQL; Rust analogue)
use std::collections::HashMap;

fn market_analysis_i(
    users: Vec<(i32, String, String)>,
    orders: Vec<(i32, String, i32, i32, i32)>,
) -> Vec<(i32, String, i32)> {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for (_, date, _, buyer, _) in orders {
        if date.starts_with("2019") {
            *cnt.entry(buyer).or_insert(0) += 1;
        }
    }
    let mut ans: Vec<(i32, String, i32)> = users
        .into_iter()
        .map(|(id, join, _)| (id, join, *cnt.get(&id).unwrap_or(&0)))
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::market_analysis_i;

    #[test]
    fn example() {
        let users = vec![
            (1, "2018-01-01".into(), "Lenovo".into()),
            (2, "2018-02-09".into(), "Samsung".into()),
            (3, "2018-01-19".into(), "LG".into()),
            (4, "2018-05-21".into(), "HP".into()),
        ];
        let orders = vec![
            (1, "2019-08-01".into(), 4, 1, 2),
            (2, "2018-08-02".into(), 2, 1, 3),
            (3, "2019-08-03".into(), 3, 2, 3),
            (4, "2018-08-04".into(), 1, 4, 2),
            (5, "2018-08-04".into(), 1, 3, 4),
            (6, "2019-08-05".into(), 2, 2, 4),
        ];
        assert_eq!(
            market_analysis_i(users, orders),
            vec![
                (1, "2018-01-01".into(), 1),
                (2, "2018-02-09".into(), 2),
                (3, "2018-01-19".into(), 0),
                (4, "2018-05-21".into(), 0),
            ]
        );
    }
}
