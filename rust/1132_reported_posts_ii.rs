/// LeetCode #1132 - Reported Posts II (SQL; Rust analogue)

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

use std::collections::{HashMap, HashSet};

fn reported_posts_ii(
    actions: Vec<(i32, i32, String, String, Option<String>)>,
    removals: Vec<(i32, String)>,
) -> f64 {
    let removed: HashSet<i32> = removals.into_iter().map(|(id, _)| id).collect();
    let mut by_day: HashMap<String, HashSet<i32>> = HashMap::new();
    for (_, post_id, date, action, extra) in actions {
        if action == "report" && extra.as_deref() == Some("spam") {
            by_day.entry(date).or_default().insert(post_id);
        }
    }
    if by_day.is_empty() {
        return 0.0;
    }
    let mut percents = Vec::new();
    for posts in by_day.values() {
        let rem = posts.iter().filter(|p| removed.contains(p)).count();
        percents.push(rem as f64 / posts.len() as f64 * 100.0);
    }
    round2(percents.iter().sum::<f64>() / percents.len() as f64)
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::reported_posts_ii;

    #[test]
    fn example() {
        let actions = vec![
            (1, 1, "2019-07-01".into(), "view".into(), None),
            (1, 1, "2019-07-01".into(), "like".into(), None),
            (1, 1, "2019-07-01".into(), "share".into(), None),
            (2, 2, "2019-07-04".into(), "view".into(), None),
            (2, 2, "2019-07-04".into(), "report".into(), Some("spam".into())),
            (3, 4, "2019-07-04".into(), "view".into(), None),
            (3, 4, "2019-07-04".into(), "report".into(), Some("spam".into())),
            (4, 3, "2019-07-02".into(), "view".into(), None),
            (4, 3, "2019-07-02".into(), "report".into(), Some("spam".into())),
            (5, 2, "2019-07-03".into(), "view".into(), None),
            (5, 2, "2019-07-03".into(), "report".into(), Some("racism".into())),
            (5, 5, "2019-07-03".into(), "view".into(), None),
            (5, 5, "2019-07-03".into(), "report".into(), Some("racism".into())),
        ];
        let removals = vec![(2, "2019-07-20".into()), (3, "2019-07-18".into())];
        assert!((reported_posts_ii(actions, removals) - 75.0).abs() < 1e-9);
    }
}
