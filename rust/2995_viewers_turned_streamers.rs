/// LeetCode #2995 - Viewers Turned Streamers (SQL; Rust analogue)
use std::collections::HashMap;

fn viewers_turned_streamers(
    sessions: Vec<(i32, String, String, i32, String)>,
) -> Vec<(i32, i32)> {
    // (user_id, session_start, session_end, session_id, session_type)
    let mut by_user: HashMap<i32, Vec<(String, String)>> = HashMap::new();
    for (user_id, start, end, _, session_type) in sessions {
        by_user
            .entry(user_id)
            .or_default()
            .push((start, session_type));
    }
    let mut ans = Vec::new();
    for (user_id, mut list) in by_user {
        list.sort_by(|a, b| a.0.cmp(&b.0));
        if list[0].1 != "Viewer" {
            continue;
        }
        let streamer_count = list.iter().filter(|(_, t)| t == "Streamer").count() as i32;
        if streamer_count > 0 {
            ans.push((user_id, streamer_count));
        }
    }
    ans.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));
    ans
}

fn main() {
    let sessions = vec![
        (101, "2023-01-01 10:00:00".into(), "2023-01-01 11:00:00".into(), 1, "Viewer".into()),
        (101, "2023-01-02 10:00:00".into(), "2023-01-02 11:00:00".into(), 2, "Streamer".into()),
        (101, "2023-01-03 10:00:00".into(), "2023-01-03 11:00:00".into(), 3, "Streamer".into()),
        (102, "2023-01-01 10:00:00".into(), "2023-01-01 11:00:00".into(), 4, "Streamer".into()),
        (103, "2023-01-01 10:00:00".into(), "2023-01-01 11:00:00".into(), 5, "Streamer".into()),
        (104, "2023-01-01 10:00:00".into(), "2023-01-01 11:00:00".into(), 6, "Viewer".into()),
    ];
    println!("{:?}", viewers_turned_streamers(sessions));
}

#[cfg(test)]
mod tests {
    use super::viewers_turned_streamers;

    #[test]
    fn example() {
        let sessions = vec![
            (101, "2023-01-01 10:00:00".into(), "2023-01-01 11:00:00".into(), 1, "Viewer".into()),
            (101, "2023-01-02 10:00:00".into(), "2023-01-02 11:00:00".into(), 2, "Streamer".into()),
            (101, "2023-01-03 10:00:00".into(), "2023-01-03 11:00:00".into(), 3, "Streamer".into()),
            (102, "2023-01-01 10:00:00".into(), "2023-01-01 11:00:00".into(), 4, "Streamer".into()),
            (103, "2023-01-01 10:00:00".into(), "2023-01-01 11:00:00".into(), 5, "Streamer".into()),
            (104, "2023-01-01 10:00:00".into(), "2023-01-01 11:00:00".into(), 6, "Viewer".into()),
        ];
        assert_eq!(viewers_turned_streamers(sessions), vec![(101, 2)]);
    }
}
