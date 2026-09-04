/// LeetCode #1142 - User Activity for the Past 30 Days II (SQL; Rust analogue)

fn date_num(s: &str) -> i32 {
    let p: Vec<i32> = s
        .split(|c: char| !c.is_ascii_digit())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    let (y, m, d) = (p[0], p[1], p[2]);
    let a = (14 - m) / 12;
    let y = y + 4800 - a;
    let m = m + 12 * a - 3;
    d + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400
}

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

use std::collections::{HashMap, HashSet};

fn user_activity_ii(activity: Vec<(i32, i32, String, String)>) -> f64 {
    let end = date_num("2019-07-27");
    let mut by_user: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (uid, sid, date, _) in activity {
        let d = date_num(&date);
        if d <= end && end - d < 30 {
            by_user.entry(uid).or_default().insert(sid);
        }
    }
    if by_user.is_empty() {
        return 0.0;
    }
    let avg = by_user.values().map(|s| s.len() as f64).sum::<f64>() / by_user.len() as f64;
    round2(avg)
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::user_activity_ii;

    #[test]
    fn example() {
        let activity = vec![
            (1, 1, "2019-07-20".into(), "open_session".into()),
            (1, 1, "2019-07-20".into(), "scroll_down".into()),
            (1, 1, "2019-07-20".into(), "end_session".into()),
            (2, 4, "2019-07-20".into(), "open_session".into()),
            (2, 4, "2019-07-21".into(), "send_message".into()),
            (2, 4, "2019-07-21".into(), "end_session".into()),
            (3, 2, "2019-07-21".into(), "open_session".into()),
            (3, 2, "2019-07-21".into(), "send_message".into()),
            (3, 2, "2019-07-21".into(), "end_session".into()),
            (3, 5, "2019-07-21".into(), "open_session".into()),
            (3, 5, "2019-07-21".into(), "scroll_down".into()),
            (3, 5, "2019-07-21".into(), "end_session".into()),
            (4, 3, "2019-06-25".into(), "open_session".into()),
            (4, 3, "2019-06-25".into(), "end_session".into()),
        ];
        assert!((user_activity_ii(activity) - 1.33).abs() < 1e-9);
    }
}
