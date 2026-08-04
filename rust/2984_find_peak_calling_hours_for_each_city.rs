/// LeetCode #2984 - Find Peak Calling Hours for Each City (SQL; Rust analogue)
use std::collections::HashMap;

fn peak_calling_hours(calls: Vec<(i32, i32, String, String)>) -> Vec<(String, i32, i32)> {
    // (caller_id, recipient_id, call_time, city) — hour from call_time "YYYY-MM-DD HH:MM:SS"
    let mut counts: HashMap<(String, i32), i32> = HashMap::new();
    for (_, _, call_time, city) in calls {
        let hour: i32 = call_time[11..13].parse().unwrap();
        *counts.entry((city, hour)).or_default() += 1;
    }
    let mut max_by_city: HashMap<String, i32> = HashMap::new();
    for ((city, _), cnt) in &counts {
        let e = max_by_city.entry(city.clone()).or_insert(0);
        *e = (*e).max(*cnt);
    }
    let mut ans: Vec<_> = counts
        .into_iter()
        .filter(|((city, _), cnt)| max_by_city[city] == *cnt)
        .map(|((city, hour), cnt)| (city, hour, cnt))
        .collect();
    ans.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));
    ans
}

fn main() {
    let calls = vec![
        (8, 4, "2021-08-24 22:46:07".into(), "Houston".into()),
        (4, 8, "2021-08-24 22:57:13".into(), "Houston".into()),
        (5, 1, "2021-08-11 21:28:44".into(), "Houston".into()),
        (8, 3, "2021-08-17 22:04:15".into(), "Houston".into()),
        (11, 3, "2021-08-17 13:07:00".into(), "New York".into()),
        (8, 11, "2021-08-17 14:22:22".into(), "New York".into()),
    ];
    println!("{:?}", peak_calling_hours(calls));
}

#[cfg(test)]
mod tests {
    use super::peak_calling_hours;

    #[test]
    fn example() {
        let calls = vec![
            (8, 4, "2021-08-24 22:46:07".into(), "Houston".into()),
            (4, 8, "2021-08-24 22:57:13".into(), "Houston".into()),
            (5, 1, "2021-08-11 21:28:44".into(), "Houston".into()),
            (8, 3, "2021-08-17 22:04:15".into(), "Houston".into()),
            (11, 3, "2021-08-17 13:07:00".into(), "New York".into()),
            (8, 11, "2021-08-17 14:22:22".into(), "New York".into()),
        ];
        assert_eq!(
            peak_calling_hours(calls),
            vec![
                ("Houston".into(), 22, 3),
                ("New York".into(), 14, 1),
                ("New York".into(), 13, 1),
            ]
        );
    }
}
