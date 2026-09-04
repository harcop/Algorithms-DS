/// LeetCode #1934 - Confirmation Rate (SQL; Rust analogue)
use std::collections::HashMap;

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

fn confirmation_rate(
    signups: Vec<(i32, String)>,
    confirmations: Vec<(i32, String, String)>,
) -> Vec<(i32, f64)> {
    let mut stats: HashMap<i32, (i32, i32)> = HashMap::new();
    for (user_id, _) in &signups {
        stats.entry(*user_id).or_insert((0, 0));
    }
    for (user_id, _, action) in confirmations {
        let e = stats.entry(user_id).or_insert((0, 0));
        e.1 += 1;
        if action == "confirmed" {
            e.0 += 1;
        }
    }
    let mut ans: Vec<(i32, f64)> = stats
        .into_iter()
        .map(|(id, (ok, total))| {
            let rate = if total == 0 {
                0.0
            } else {
                round2(ok as f64 / total as f64)
            };
            (id, rate)
        })
        .collect();
    ans.sort_by_key(|t| t.0);
    ans
}

fn main() {
    println!("{:?}", confirmation_rate(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::confirmation_rate;

    #[test]
    fn example_one() {
        let signups = vec![
            (3, "2020-03-21 10:16:13".into()),
            (7, "2020-01-04 13:57:59".into()),
            (2, "2020-07-29 23:09:44".into()),
            (6, "2020-12-09 10:39:37".into()),
        ];
        let confirmations = vec![
            (3, "2021-01-06 03:30:46".into(), "timeout".into()),
            (3, "2021-07-14 14:00:00".into(), "timeout".into()),
            (7, "2021-06-12 11:57:29".into(), "confirmed".into()),
            (7, "2021-06-13 12:58:28".into(), "confirmed".into()),
            (7, "2021-06-14 13:59:27".into(), "confirmed".into()),
            (2, "2021-01-22 00:00:00".into(), "confirmed".into()),
            (2, "2021-02-28 23:59:59".into(), "timeout".into()),
        ];
        assert_eq!(
            confirmation_rate(signups, confirmations),
            vec![(2, 0.50), (3, 0.00), (6, 0.00), (7, 1.00)]
        );
    }
}
