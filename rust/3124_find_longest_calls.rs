/// LeetCode #3124 - Find Longest Calls (SQL; Rust analogue)
fn format_duration(secs: i32) -> String {
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

fn find_longest_calls(
    contacts: Vec<(i32, String, String)>,
    calls: Vec<(i32, String, i32)>,
) -> Vec<(String, String, String)> {
    // contacts: (id, first_name, last_name)
    // calls: (contact_id, type, duration)
    let names: std::collections::HashMap<i32, String> =
        contacts.into_iter().map(|(id, first, _)| (id, first)).collect();

    let mut rows: Vec<(String, String, i32)> = Vec::new();
    for (cid, typ, duration) in calls {
        if let Some(name) = names.get(&cid) {
            rows.push((name.clone(), typ, duration));
        }
    }

    let mut result = Vec::new();
    for typ in ["outgoing", "incoming"] {
        let mut group: Vec<_> = rows
            .iter()
            .filter(|(_, t, _)| t == typ)
            .cloned()
            .collect();
        group.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| b.0.cmp(&a.0)));
        for (name, t, dur) in group.into_iter().take(3) {
            result.push((name, t, format_duration(dur)));
        }
    }
    result
}

fn main() {
    let contacts = vec![
        (1, "John".into(), "Doe".into()),
        (3, "Alice".into(), "Johnson".into()),
    ];
    let calls = vec![
        (1, "outgoing".into(), 180),
        (3, "outgoing".into(), 360),
    ];
    println!("{:?}", find_longest_calls(contacts, calls));
}

#[cfg(test)]
mod tests {
    use super::find_longest_calls;

    #[test]
    fn example() {
        let contacts = vec![
            (1, "John".into(), "Doe".into()),
            (2, "Jane".into(), "Smith".into()),
            (3, "Alice".into(), "Johnson".into()),
            (4, "Michael".into(), "Brown".into()),
            (5, "Emily".into(), "Davis".into()),
        ];
        let calls = vec![
            (1, "incoming".into(), 120),
            (1, "outgoing".into(), 180),
            (2, "incoming".into(), 300),
            (2, "outgoing".into(), 240),
            (3, "incoming".into(), 150),
            (3, "outgoing".into(), 360),
            (4, "incoming".into(), 420),
            (4, "outgoing".into(), 200),
            (5, "incoming".into(), 180),
            (5, "outgoing".into(), 280),
        ];
        assert_eq!(
            find_longest_calls(contacts, calls),
            vec![
                ("Alice".into(), "outgoing".into(), "00:06:00".into()),
                ("Emily".into(), "outgoing".into(), "00:04:40".into()),
                ("Jane".into(), "outgoing".into(), "00:04:00".into()),
                ("Michael".into(), "incoming".into(), "00:07:00".into()),
                ("Jane".into(), "incoming".into(), "00:05:00".into()),
                ("Emily".into(), "incoming".into(), "00:03:00".into()),
            ]
        );
    }
}
