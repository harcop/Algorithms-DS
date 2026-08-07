/// LeetCode #3056 - Snaps Analysis (SQL; Rust analogue)
use std::collections::HashMap;

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

fn snaps_analysis(
    activities: Vec<(i32, i32, String, f64)>,
    age: Vec<(i32, String)>,
) -> Vec<(String, f64, f64)> {
    let bucket_by_user: HashMap<i32, String> = age.into_iter().collect();
    let mut send: HashMap<String, f64> = HashMap::new();
    let mut open: HashMap<String, f64> = HashMap::new();

    for (_, user_id, activity_type, time_spent) in activities {
        let Some(bucket) = bucket_by_user.get(&user_id) else {
            continue;
        };
        match activity_type.as_str() {
            "send" => *send.entry(bucket.clone()).or_default() += time_spent,
            "open" => *open.entry(bucket.clone()).or_default() += time_spent,
            _ => {}
        }
    }

    let mut ans: Vec<_> = send
        .keys()
        .chain(open.keys())
        .cloned()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .map(|bucket| {
            let s = send.get(&bucket).copied().unwrap_or(0.0);
            let o = open.get(&bucket).copied().unwrap_or(0.0);
            let total = s + o;
            (
                bucket,
                round2(100.0 * s / total),
                round2(100.0 * o / total),
            )
        })
        .collect();
    ans.sort_by(|a, b| a.0.cmp(&b.0));
    ans
}

fn example_activities() -> Vec<(i32, i32, String, f64)> {
    vec![
        (7274, 123, "open".into(), 4.50),
        (2425, 123, "send".into(), 3.50),
        (1413, 456, "send".into(), 5.67),
        (2536, 456, "open".into(), 3.00),
        (8564, 456, "send".into(), 8.24),
        (5235, 789, "send".into(), 6.24),
        (4251, 123, "open".into(), 1.25),
        (1435, 789, "open".into(), 5.25),
    ]
}

fn example_age() -> Vec<(i32, String)> {
    vec![
        (123, "31-35".into()),
        (789, "21-25".into()),
        (456, "26-30".into()),
    ]
}

fn main() {
    println!(
        "{:?}",
        snaps_analysis(example_activities(), example_age())
    );
}

#[cfg(test)]
mod tests {
    use super::{example_activities, example_age, snaps_analysis};

    fn expected() -> Vec<(String, f64, f64)> {
        vec![
            ("21-25".into(), 54.31, 45.69),
            ("26-30".into(), 82.26, 17.74),
            ("31-35".into(), 37.84, 62.16),
        ]
    }

    #[test]
    fn example() {
        let got = snaps_analysis(example_activities(), example_age());
        assert_eq!(got, expected());
    }
}
