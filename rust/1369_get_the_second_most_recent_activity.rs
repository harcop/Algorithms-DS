/// LeetCode #1369 - Get the Second Most Recent Activity (SQL; Rust analogue)
use std::collections::HashMap;

fn second_most_recent(
    activity: Vec<(String, String, String, String)>,
) -> Vec<(String, String, String, String)> {
    let mut by_user: HashMap<String, Vec<(String, String, String)>> = HashMap::new();
    for (user, act, start, end) in activity {
        by_user.entry(user).or_default().push((act, start, end));
    }
    let mut ans = Vec::new();
    for (user, mut rows) in by_user {
        rows.sort_by(|a, b| b.1.cmp(&a.1));
        let pick = if rows.len() == 1 {
            rows.remove(0)
        } else {
            rows.remove(1)
        };
        ans.push((user, pick.0, pick.1, pick.2));
    }
    ans
}

fn main() {
    println!("{:?}", second_most_recent(vec![]));
}

#[cfg(test)]
mod tests {
    use super::second_most_recent;

    #[test]
    fn example() {
        let activity = vec![
            ("Alice".into(), "Travel".into(), "2020-02-12".into(), "2020-02-20".into()),
            ("Alice".into(), "Dancing".into(), "2020-02-21".into(), "2020-02-23".into()),
            ("Alice".into(), "Travel".into(), "2020-02-24".into(), "2020-02-28".into()),
            ("Bob".into(), "Travel".into(), "2020-02-11".into(), "2020-02-18".into()),
        ];
        let mut got = second_most_recent(activity);
        got.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(
            got,
            vec![
                ("Alice".into(), "Dancing".into(), "2020-02-21".into(), "2020-02-23".into()),
                ("Bob".into(), "Travel".into(), "2020-02-11".into(), "2020-02-18".into()),
            ]
        );
    }
}
