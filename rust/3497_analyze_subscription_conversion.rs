/// LeetCode #3497 - Analyze Subscription Conversion (SQL; Rust analogue)
use std::collections::HashMap;

fn round2(x: f64) -> String {
    format!("{:.2}", (x * 100.0).round() / 100.0)
}

fn analyze_subscription_conversion(
    user_activity: Vec<(i32, String, String, i32)>,
) -> Vec<(i32, String, String)> {
    let mut trial: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut paid: HashMap<i32, Vec<i32>> = HashMap::new();
    for (user_id, _, activity_type, duration) in &user_activity {
        match activity_type.as_str() {
            "free_trial" => trial.entry(*user_id).or_default().push(*duration),
            "paid" => paid.entry(*user_id).or_default().push(*duration),
            _ => {}
        }
    }
    let mut ids: Vec<i32> = trial
        .keys()
        .filter(|id| paid.contains_key(id))
        .copied()
        .collect();
    ids.sort_unstable();
    ids.into_iter()
        .map(|id| {
            let t = &trial[&id];
            let p = &paid[&id];
            let trial_avg = t.iter().sum::<i32>() as f64 / t.len() as f64;
            let paid_avg = p.iter().sum::<i32>() as f64 / p.len() as f64;
            (id, round2(trial_avg), round2(paid_avg))
        })
        .collect()
}

fn main() {
    let rows = vec![
        (1, "2023-01-01".into(), "free_trial".into(), 45),
        (1, "2023-01-10".into(), "paid".into(), 75),
    ];
    println!("{:?}", analyze_subscription_conversion(rows));
}

#[cfg(test)]
mod tests {
    use super::analyze_subscription_conversion;

    #[test]
    fn example() {
        let rows = vec![
            (1, "2023-01-01".into(), "free_trial".into(), 45),
            (1, "2023-01-02".into(), "free_trial".into(), 30),
            (1, "2023-01-05".into(), "free_trial".into(), 60),
            (1, "2023-01-10".into(), "paid".into(), 75),
            (1, "2023-01-12".into(), "paid".into(), 90),
            (1, "2023-01-15".into(), "paid".into(), 65),
            (2, "2023-02-01".into(), "free_trial".into(), 55),
            (2, "2023-02-03".into(), "free_trial".into(), 25),
            (2, "2023-02-07".into(), "free_trial".into(), 50),
            (2, "2023-02-10".into(), "cancelled".into(), 0),
            (3, "2023-03-05".into(), "free_trial".into(), 70),
            (3, "2023-03-06".into(), "free_trial".into(), 60),
            (3, "2023-03-08".into(), "free_trial".into(), 80),
            (3, "2023-03-12".into(), "paid".into(), 50),
            (3, "2023-03-15".into(), "paid".into(), 55),
            (3, "2023-03-20".into(), "paid".into(), 85),
            (4, "2023-04-01".into(), "free_trial".into(), 40),
            (4, "2023-04-03".into(), "free_trial".into(), 35),
            (4, "2023-04-05".into(), "paid".into(), 45),
            (4, "2023-04-07".into(), "cancelled".into(), 0),
        ];
        assert_eq!(
            analyze_subscription_conversion(rows),
            vec![
                (1, "45.00".into(), "76.67".into()),
                (3, "70.00".into(), "63.33".into()),
                (4, "37.50".into(), "45.00".into()),
            ]
        );
    }
}
