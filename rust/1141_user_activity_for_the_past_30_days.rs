/// LeetCode #1141 - User Activity for the Past 30 Days I (SQL; Rust analogue)

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

use std::collections::{HashMap, HashSet};

fn user_activity_i(activity: Vec<(i32, i32, String, String)>) -> Vec<(String, i32)> {
    let end = date_num("2019-07-27");
    let mut by_day: HashMap<String, HashSet<i32>> = HashMap::new();
    for (uid, _, date, _) in activity {
        let d = date_num(&date);
        if d <= end && end - d < 30 {
            by_day.entry(date).or_default().insert(uid);
        }
    }
    let mut ans: Vec<(String, i32)> = by_day
        .into_iter()
        .map(|(d, u)| (d, u.len() as i32))
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::user_activity_i;

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
            (4, 3, "2019-06-25".into(), "open_session".into()),
            (4, 3, "2019-06-25".into(), "end_session".into()),
        ];
        assert_eq!(
            user_activity_i(activity),
            vec![("2019-07-20".into(), 2), ("2019-07-21".into(), 2)]
        );
    }
}
