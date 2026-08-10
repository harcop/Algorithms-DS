/// LeetCode #3126 - Server Utilization Time (SQL; Rust analogue)
use std::collections::HashMap;

fn total_uptime_days(servers: Vec<(i32, String, String)>) -> i64 {
    // (server_id, status_time as "YYYY-MM-DD HH:MM:SS", session_status)
    let mut by_server: HashMap<i32, Vec<(i64, String)>> = HashMap::new();
    for (sid, time, status) in servers {
        let secs = parse_datetime(&time);
        by_server.entry(sid).or_default().push((secs, status));
    }
    let mut total = 0i64;
    for events in by_server.values_mut() {
        events.sort_by_key(|(t, _)| *t);
        for i in 0..events.len() {
            if events[i].1 == "start" {
                if let Some((next_t, _)) = events.get(i + 1) {
                    total += next_t - events[i].0;
                }
            }
        }
    }
    total / 86400
}

fn parse_datetime(s: &str) -> i64 {
    // naive: days since epoch-ish for Nov 2023 relative comparisons
    // Parse "YYYY-MM-DD HH:MM:SS"
    let parts: Vec<&str> = s.split(|c| c == '-' || c == ' ' || c == ':').collect();
    let y: i64 = parts[0].parse().unwrap();
    let mo: i64 = parts[1].parse().unwrap();
    let d: i64 = parts[2].parse().unwrap();
    let h: i64 = parts[3].parse().unwrap();
    let mi: i64 = parts[4].parse().unwrap();
    let se: i64 = parts[5].parse().unwrap();
    // days from year start approx (enough for same-year diffs)
    let month_days = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut days = (y - 1970) * 365 + (y - 1969) / 4;
    for m in 1..mo as usize {
        days += month_days[m];
    }
    days += d - 1;
    days * 86400 + h * 3600 + mi * 60 + se
}

fn main() {
    println!("{}", total_uptime_days(vec![]));
}

#[cfg(test)]
mod tests {
    use super::total_uptime_days;

    #[test]
    fn example() {
        let servers = vec![
            (3, "2023-11-04 16:29:47".into(), "start".into()),
            (3, "2023-11-05 01:49:47".into(), "stop".into()),
            (3, "2023-11-25 01:37:08".into(), "start".into()),
            (3, "2023-11-25 03:50:08".into(), "stop".into()),
            (1, "2023-11-13 03:05:31".into(), "start".into()),
            (1, "2023-11-13 11:10:31".into(), "stop".into()),
            (4, "2023-11-29 15:11:17".into(), "start".into()),
            (4, "2023-11-29 15:42:17".into(), "stop".into()),
            (4, "2023-11-20 00:31:44".into(), "start".into()),
            (4, "2023-11-20 07:03:44".into(), "stop".into()),
            (1, "2023-11-20 00:27:11".into(), "start".into()),
            (1, "2023-11-20 01:41:11".into(), "stop".into()),
            (3, "2023-11-04 23:16:48".into(), "start".into()),
            (3, "2023-11-05 01:15:48".into(), "stop".into()),
            (4, "2023-11-30 15:09:18".into(), "start".into()),
            (4, "2023-11-30 20:48:18".into(), "stop".into()),
            (4, "2023-11-25 21:09:06".into(), "start".into()),
            (4, "2023-11-26 04:58:06".into(), "stop".into()),
            (5, "2023-11-16 19:42:22".into(), "start".into()),
            (5, "2023-11-16 21:08:22".into(), "stop".into()),
        ];
        assert_eq!(total_uptime_days(servers), 1);
    }
}
