/// LeetCode #3832 - Find Users with Persistent Behavior Patterns (SQL; Rust analogue)
use std::collections::HashMap;

fn date_to_days(s: &str) -> i32 {
    let y: i32 = s[0..4].parse().unwrap();
    let m: i32 = s[5..7].parse().unwrap();
    let d: i32 = s[8..10].parse().unwrap();
    let (mut y, mut m) = (y, m);
    if m <= 2 {
        y -= 1;
        m += 12;
    }
    365 * y + y / 4 - y / 100 + y / 400 + (153 * m - 457) / 5 + d
}

/// row: (user_id, action_date, action)
fn find_behaviorally_stable_users(
    activity: Vec<(i32, String, String)>,
) -> Vec<(i32, String, i32, String, String)> {
    let mut by_day: HashMap<(i32, String), Vec<(String, String)>> = HashMap::new();
    for (uid, date, action) in activity {
        by_day.entry((uid, date)).or_default().push((action, String::new()));
    }
    let mut rows = Vec::new();
    for ((uid, date), acts) in by_day {
        if acts.len() != 1 {
            continue;
        }
        rows.push((uid, date, acts[0].0.clone()));
    }
    rows.sort_by(|a, b| a.0.cmp(&b.0).then(a.2.cmp(&b.2)).then(a.1.cmp(&b.1)));
    let mut best: HashMap<i32, (String, i32, String, String)> = HashMap::new();
    let mut i = 0;
    while i < rows.len() {
        let uid = rows[i].0;
        let action = rows[i].2.clone();
        let mut j = i + 1;
        while j < rows.len() && rows[j].0 == uid && rows[j].2 == action {
            j += 1;
        }
        let mut start = i;
        for k in i..j {
            if k > start {
                let prev = date_to_days(&rows[k - 1].1);
                let cur = date_to_days(&rows[k].1);
                if cur != prev + 1 {
                    let len = (k - start) as i32;
                    if len >= 5 {
                        best.entry(uid).and_modify(|e| {
                            if len > e.1 {
                                *e = (action.clone(), len, rows[start].1.clone(), rows[k - 1].1.clone());
                            }
                        }).or_insert((
                            action.clone(),
                            len,
                            rows[start].1.clone(),
                            rows[k - 1].1.clone(),
                        ));
                    }
                    start = k;
                }
            }
        }
        let len = (j - start) as i32;
        if len >= 5 {
            best.entry(uid).and_modify(|e| {
                if len > e.1 {
                    *e = (action.clone(), len, rows[start].1.clone(), rows[j - 1].1.clone());
                }
            }).or_insert((
                action.clone(),
                len,
                rows[start].1.clone(),
                rows[j - 1].1.clone(),
            ));
        }
        i = j;
    }
    let mut ans: Vec<(i32, String, i32, String, String)> = best
        .into_iter()
        .map(|(uid, (action, len, start, end))| (uid, action, len, start, end))
        .collect();
    ans.sort_by(|a, b| b.2.cmp(&a.2).then(a.0.cmp(&b.0)));
    ans
}

fn main() {
    println!("{:?}", find_behaviorally_stable_users(vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_behaviorally_stable_users;

    #[test]
    fn example() {
        let rows = vec![
            (1, "2024-01-01".into(), "login".into()),
            (1, "2024-01-02".into(), "login".into()),
            (1, "2024-01-03".into(), "login".into()),
            (1, "2024-01-04".into(), "login".into()),
            (1, "2024-01-05".into(), "login".into()),
            (1, "2024-01-06".into(), "logout".into()),
            (2, "2024-01-01".into(), "click".into()),
            (2, "2024-01-02".into(), "click".into()),
            (2, "2024-01-03".into(), "click".into()),
            (2, "2024-01-04".into(), "click".into()),
            (3, "2024-01-01".into(), "view".into()),
            (3, "2024-01-02".into(), "view".into()),
            (3, "2024-01-03".into(), "view".into()),
            (3, "2024-01-04".into(), "view".into()),
            (3, "2024-01-05".into(), "view".into()),
            (3, "2024-01-06".into(), "view".into()),
            (3, "2024-01-07".into(), "view".into()),
        ];
        let ans = find_behaviorally_stable_users(rows);
        assert_eq!(ans.len(), 2);
        assert_eq!(ans[0].0, 3);
        assert_eq!(ans[0].1, "view");
        assert_eq!(ans[0].2, 7);
        assert_eq!(ans[0].3, "2024-01-01");
        assert_eq!(ans[0].4, "2024-01-07");
        assert_eq!(ans[1].0, 1);
        assert_eq!(ans[1].1, "login");
        assert_eq!(ans[1].2, 5);
        assert_eq!(ans[1].3, "2024-01-01");
        assert_eq!(ans[1].4, "2024-01-05");
    }
}
