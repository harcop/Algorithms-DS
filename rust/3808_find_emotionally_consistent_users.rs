/// LeetCode #3808 - Find Emotionally Consistent Users (SQL; Rust analogue)
use std::collections::HashMap;

/// row: (user_id, content_id, reaction)
fn find_emotionally_consistent_users(
    reactions: Vec<(i32, i32, String)>,
) -> Vec<(i32, String, f64)> {
    let mut cnt: HashMap<i32, HashMap<String, i32>> = HashMap::new();
    for (uid, _cid, reaction) in reactions {
        *cnt.entry(uid)
            .or_default()
            .entry(reaction)
            .or_insert(0) += 1;
    }
    let mut ans = Vec::new();
    for (uid, rec) in cnt {
        let total: i32 = rec.values().sum();
        if total < 5 {
            continue;
        }
        let (dom, mx) = rec.into_iter().max_by_key(|(_, c)| *c).unwrap();
        let ratio = (mx as f64 / total as f64 * 100.0).round() / 100.0;
        if ratio >= 0.60 {
            ans.push((uid, dom, ratio));
        }
    }
    ans.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap().then(a.0.cmp(&b.0)));
    ans
}

fn main() {
    println!("{:?}", find_emotionally_consistent_users(vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_emotionally_consistent_users;

    #[test]
    fn example() {
        let rows = vec![
            (1, 101, "like".into()),
            (1, 102, "like".into()),
            (1, 103, "like".into()),
            (1, 104, "wow".into()),
            (1, 105, "like".into()),
            (2, 201, "like".into()),
            (2, 202, "wow".into()),
            (2, 203, "sad".into()),
            (2, 204, "like".into()),
            (2, 205, "wow".into()),
            (3, 301, "love".into()),
            (3, 302, "love".into()),
            (3, 303, "love".into()),
            (3, 304, "love".into()),
            (3, 305, "love".into()),
        ];
        let ans = find_emotionally_consistent_users(rows);
        assert_eq!(ans.len(), 2);
        assert_eq!(ans[0].0, 3);
        assert_eq!(ans[0].1, "love");
        assert!((ans[0].2 - 1.00).abs() < 1e-9);
        assert_eq!(ans[1].0, 1);
        assert_eq!(ans[1].1, "like");
        assert!((ans[1].2 - 0.80).abs() < 1e-9);
    }
}
