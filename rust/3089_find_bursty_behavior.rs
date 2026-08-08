/// LeetCode #3089 - Find Bursty Behavior (SQL; Rust analogue)
use std::collections::BTreeMap;

fn find_bursty_behavior(posts: Vec<(i32, i32, String)>) -> Vec<(i32, i32, f64)> {
    // (post_id, user_id, post_date YYYY-MM-DD)
    let mut by_user: BTreeMap<i32, Vec<i32>> = BTreeMap::new();

    for (_, user_id, date) in posts {
        if !date.starts_with("2024-02") {
            continue;
        }
        let day: i32 = date[8..10].parse().unwrap();
        by_user.entry(user_id).or_default().push(day);
    }

    let mut result = Vec::new();
    for (user_id, mut days) in by_user {
        days.sort_unstable();
        let total = days.len() as f64;
        let avg_weekly = total / 4.0;

        let mut max_7 = 0i32;
        for &start in &days {
            let cnt = days.iter().filter(|&&d| d >= start && d <= start + 6).count() as i32;
            max_7 = max_7.max(cnt);
        }

        if (max_7 as f64) >= avg_weekly * 2.0 {
            result.push((user_id, max_7, avg_weekly));
        }
    }
    result
}

fn main() {
    let posts = vec![
        (1, 1, "2024-02-27".into()),
        (2, 5, "2024-02-06".into()),
        (3, 3, "2024-02-25".into()),
        (4, 3, "2024-02-14".into()),
        (5, 3, "2024-02-06".into()),
        (6, 2, "2024-02-25".into()),
    ];
    println!("{:?}", find_bursty_behavior(posts));
}

#[cfg(test)]
mod tests {
    use super::find_bursty_behavior;

    #[test]
    fn example() {
        let posts = vec![
            (1, 1, "2024-02-27".into()),
            (2, 5, "2024-02-06".into()),
            (3, 3, "2024-02-25".into()),
            (4, 3, "2024-02-14".into()),
            (5, 3, "2024-02-06".into()),
            (6, 2, "2024-02-25".into()),
        ];
        let got = find_bursty_behavior(posts);
        assert_eq!(got.len(), 3);
        assert_eq!(got[0].0, 1);
        assert_eq!(got[0].1, 1);
        assert!((got[0].2 - 0.25).abs() < 1e-9);
        assert_eq!(got[1].0, 2);
        assert_eq!(got[2].0, 5);
    }
}
