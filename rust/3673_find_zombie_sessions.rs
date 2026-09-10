/// LeetCode #3673 - Find Zombie Sessions (SQL; Rust analogue)
use std::collections::HashMap;

fn find_zombie_sessions(
    events: Vec<(i32, i32, i32, String, String)>,
) -> Vec<(String, i32, i32, i32)> {
    // (event_id, user_id, minute_offset, event_type, session_id)
    struct Agg {
        user_id: i32,
        min_t: i32,
        max_t: i32,
        scrolls: i32,
        clicks: i32,
        purchases: i32,
    }
    let mut map: HashMap<String, Agg> = HashMap::new();
    for (_eid, uid, t, ty, sid) in events {
        let e = map.entry(sid).or_insert(Agg {
            user_id: uid,
            min_t: t,
            max_t: t,
            scrolls: 0,
            clicks: 0,
            purchases: 0,
        });
        e.min_t = e.min_t.min(t);
        e.max_t = e.max_t.max(t);
        match ty.as_str() {
            "scroll" => e.scrolls += 1,
            "click" => e.clicks += 1,
            "purchase" => e.purchases += 1,
            _ => {}
        }
    }
    let mut ans: Vec<(String, i32, i32, i32)> = map
        .into_iter()
        .filter(|(_, a)| {
            a.max_t - a.min_t >= 30
                && a.scrolls >= 5
                && a.purchases == 0
                && (a.clicks as f64) / (a.scrolls as f64) < 0.2
        })
        .map(|(sid, a)| (sid, a.user_id, a.max_t - a.min_t, a.scrolls))
        .collect();
    ans.sort_by(|a, b| b.3.cmp(&a.3).then(a.0.cmp(&b.0)));
    ans
}

fn main() {
    println!("{:?}", find_zombie_sessions(vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_zombie_sessions;

    #[test]
    fn example() {
        let events = vec![
            (1, 201, 0, "app_open".into(), "S001".into()),
            (2, 201, 5, "scroll".into(), "S001".into()),
            (3, 201, 10, "scroll".into(), "S001".into()),
            (4, 201, 15, "scroll".into(), "S001".into()),
            (5, 201, 20, "scroll".into(), "S001".into()),
            (6, 201, 25, "scroll".into(), "S001".into()),
            (7, 201, 30, "scroll".into(), "S001".into()),
            (8, 201, 35, "app_close".into(), "S001".into()),
            (9, 202, 60, "app_open".into(), "S002".into()),
            (10, 202, 62, "click".into(), "S002".into()),
            (11, 202, 65, "scroll".into(), "S002".into()),
            (12, 202, 68, "click".into(), "S002".into()),
            (13, 202, 70, "scroll".into(), "S002".into()),
            (14, 202, 75, "purchase".into(), "S002".into()),
            (15, 202, 80, "app_close".into(), "S002".into()),
            (16, 203, 120, "app_open".into(), "S003".into()),
            (17, 203, 130, "scroll".into(), "S003".into()),
            (18, 203, 140, "scroll".into(), "S003".into()),
            (19, 203, 145, "click".into(), "S003".into()),
            (20, 203, 150, "scroll".into(), "S003".into()),
            (21, 203, 160, "scroll".into(), "S003".into()),
            (22, 203, 170, "scroll".into(), "S003".into()),
            (23, 203, 180, "app_close".into(), "S003".into()),
            (24, 204, 240, "app_open".into(), "S004".into()),
            (25, 204, 245, "scroll".into(), "S004".into()),
            (26, 204, 248, "scroll".into(), "S004".into()),
            (27, 204, 250, "click".into(), "S004".into()),
            (28, 204, 252, "app_close".into(), "S004".into()),
        ];
        assert_eq!(
            find_zombie_sessions(events),
            vec![("S001".into(), 201, 35, 6)]
        );
    }
}
