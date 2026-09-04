/// LeetCode #1113 - Reported Posts (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn reported_posts(
    actions: Vec<(i32, i32, String, String, Option<String>)>,
) -> Vec<(String, i32)> {
    let mut by_reason: HashMap<String, HashSet<i32>> = HashMap::new();
    for (_, post_id, date, action, extra) in actions {
        if date == "2019-07-04" && action == "report" {
            if let Some(reason) = extra {
                by_reason.entry(reason).or_default().insert(post_id);
            }
        }
    }
    let mut ans: Vec<(String, i32)> = by_reason
        .into_iter()
        .map(|(r, ps)| (r, ps.len() as i32))
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::reported_posts;

    #[test]
    fn example() {
        let actions = vec![
            (1, 1, "2019-07-01".into(), "view".into(), None),
            (1, 1, "2019-07-01".into(), "like".into(), None),
            (1, 1, "2019-07-01".into(), "share".into(), None),
            (2, 4, "2019-07-04".into(), "view".into(), None),
            (2, 4, "2019-07-04".into(), "report".into(), Some("spam".into())),
            (3, 4, "2019-07-04".into(), "view".into(), None),
            (3, 4, "2019-07-04".into(), "report".into(), Some("spam".into())),
            (4, 3, "2019-07-02".into(), "view".into(), None),
            (4, 3, "2019-07-02".into(), "report".into(), Some("spam".into())),
            (5, 2, "2019-07-04".into(), "view".into(), None),
            (5, 2, "2019-07-04".into(), "report".into(), Some("racism".into())),
            (5, 5, "2019-07-04".into(), "view".into(), None),
            (5, 5, "2019-07-04".into(), "report".into(), Some("racism".into())),
        ];
        assert_eq!(
            reported_posts(actions),
            vec![("racism".into(), 2), ("spam".into(), 1)]
        );
    }
}
