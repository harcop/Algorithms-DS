/// LeetCode #3060 - User Activities within Time Bounds (SQL; Rust analogue)

fn parse_datetime(s: &str) -> i64 {
    // "YYYY-MM-DD HH:MM:SS" -> unix-like seconds from epoch components
    let parts: Vec<&str> = s.split(['-', ' ', ':']).collect();
    let y: i64 = parts[0].parse().unwrap();
    let mo: i64 = parts[1].parse().unwrap();
    let d: i64 = parts[2].parse().unwrap();
    let h: i64 = parts[3].parse().unwrap();
    let mi: i64 = parts[4].parse().unwrap();
    let sec: i64 = parts[5].parse().unwrap();
    (((y * 12 + mo) * 31 + d) * 24 + h) * 3600 + mi * 60 + sec
}

fn timestamp_diff_hours(prev_end: i64, next_start: i64) -> i64 {
    (next_start - prev_end) / 3600
}

fn user_activities_within_bounds(
    sessions: Vec<(i32, String, String, i32, String)>,
) -> Vec<i32> {
    use std::collections::{HashMap, HashSet};

    let mut by_user_type: HashMap<(i32, String), Vec<(i64, i64)>> = HashMap::new();

    for (user_id, start, end, _session_id, session_type) in sessions {
        by_user_type
            .entry((user_id, session_type))
            .or_default()
            .push((parse_datetime(&start), parse_datetime(&end)));
    }

    let mut ans = HashSet::new();

    for ((user_id, _), mut times) in by_user_type {
        times.sort_by_key(|(_, end)| *end);
        for w in times.windows(2) {
            let prev_end = w[0].1;
            let next_start = w[1].0;
            if timestamp_diff_hours(prev_end, next_start) <= 12 {
                ans.insert(user_id);
                break;
            }
        }
    }

    let mut result: Vec<_> = ans.into_iter().collect();
    result.sort_unstable();
    result
}

fn example_sessions() -> Vec<(i32, String, String, i32, String)> {
    vec![
        (101, "2023-11-01 08:00:00".into(), "2023-11-01 09:00:00".into(), 1, "Viewer".into()),
        (101, "2023-11-01 10:00:00".into(), "2023-11-01 11:00:00".into(), 2, "Streamer".into()),
        (102, "2023-11-01 13:00:00".into(), "2023-11-01 14:00:00".into(), 3, "Viewer".into()),
        (102, "2023-11-01 15:00:00".into(), "2023-11-01 16:00:00".into(), 4, "Viewer".into()),
        (101, "2023-11-02 09:00:00".into(), "2023-11-02 10:00:00".into(), 5, "Viewer".into()),
        (102, "2023-11-02 12:00:00".into(), "2023-11-02 13:00:00".into(), 6, "Streamer".into()),
        (101, "2023-11-02 13:00:00".into(), "2023-11-02 14:00:00".into(), 7, "Streamer".into()),
        (102, "2023-11-02 16:00:00".into(), "2023-11-02 17:00:00".into(), 8, "Viewer".into()),
        (103, "2023-11-01 08:00:00".into(), "2023-11-01 09:00:00".into(), 9, "Viewer".into()),
        (103, "2023-11-02 20:00:00".into(), "2023-11-02 23:00:00".into(), 10, "Viewer".into()),
        (103, "2023-11-03 09:00:00".into(), "2023-11-03 10:00:00".into(), 11, "Viewer".into()),
    ]
}

fn main() {
    println!("{:?}", user_activities_within_bounds(example_sessions()));
}

#[cfg(test)]
mod tests {
    use super::{example_sessions, user_activities_within_bounds};

    #[test]
    fn example() {
        assert_eq!(user_activities_within_bounds(example_sessions()), vec![102, 103]);
    }
}
