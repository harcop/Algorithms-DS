/// LeetCode #1633 - Percentage of Users Attended a Contest (SQL; Rust analogue)
use std::collections::HashMap;

fn contest_percentage(
    users: Vec<(i32, String)>,
    register: Vec<(i32, i32)>,
) -> Vec<(i32, f64)> {
    let n = users.len() as f64;
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for (contest, _) in register {
        *cnt.entry(contest).or_insert(0) += 1;
    }
    let mut ans: Vec<(i32, f64)> = cnt
        .into_iter()
        .map(|(id, c)| (id, (c as f64 / n * 10000.0).round() / 100.0))
        .collect();
    ans.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(a.0.cmp(&b.0)));
    ans
}

fn main() {
    println!("{:?}", contest_percentage(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::contest_percentage;

    #[test]
    fn example() {
        let users = vec![
            (6, "Alice".into()),
            (2, "Bob".into()),
            (7, "Alex".into()),
        ];
        let register = vec![
            (215, 6),
            (209, 2),
            (208, 2),
            (210, 6),
            (208, 6),
            (209, 7),
            (209, 6),
            (215, 7),
            (208, 7),
            (210, 2),
            (207, 2),
            (210, 7),
        ];
        assert_eq!(
            contest_percentage(users, register),
            vec![
                (208, 100.0),
                (209, 100.0),
                (210, 100.0),
                (215, 66.67),
                (207, 33.33),
            ]
        );
    }
}
