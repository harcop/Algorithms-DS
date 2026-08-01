use std::collections::BTreeMap;

/// LeetCode #2854 - Rolling Average Steps
fn rolling_average_steps(rows: Vec<(i32, i32, &str)>) -> Vec<(i32, String, f64)> {
    let mut users: BTreeMap<i32, Vec<(i32, String, i32)>> = BTreeMap::new();
    for (user_id, steps_count, steps_date) in rows {
        users.entry(user_id).or_default().push((
            to_days(steps_date),
            steps_date.to_string(),
            steps_count,
        ));
    }

    let mut answer = Vec::new();
    for (user_id, days) in &mut users {
        days.sort_unstable_by_key(|row| row.0);
        for window in days.windows(3) {
            if window[1].0 == window[0].0 + 1 && window[2].0 == window[1].0 + 1 {
                let average =
                    (window.iter().map(|row| row.2).sum::<i32>() as f64 / 3.0 * 100.0).round()
                        / 100.0;
                answer.push((*user_id, window[2].1.clone(), average));
            }
        }
    }
    answer
}

fn to_days(date: &str) -> i32 {
    let parts: Vec<i32> = date
        .split('-')
        .map(|part| part.parse().unwrap())
        .collect();
    let (year, month, day) = (parts[0], parts[1], parts[2]);
    let a = (14 - month) / 12;
    let y = year + 4800 - a;
    let m = month + 12 * a - 3;
    day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
}

fn main() {
    let rows = vec![
        (1, 395, "2021-09-04"),
        (1, 499, "2021-09-05"),
        (1, 712, "2021-09-06"),
        (1, 576, "2021-09-07"),
    ];
    println!("{:?}", rolling_average_steps(rows));
}

#[cfg(test)]
mod tests {
    use super::rolling_average_steps;

    #[test]
    fn example_one() {
        let rows = vec![
            (1, 687, "2021-09-02"),
            (1, 395, "2021-09-04"),
            (1, 499, "2021-09-05"),
            (1, 712, "2021-09-06"),
            (1, 576, "2021-09-07"),
            (2, 153, "2021-09-06"),
            (2, 171, "2021-09-07"),
            (3, 665, "2021-09-07"),
            (3, 337, "2021-09-08"),
            (3, 515, "2021-09-09"),
            (3, 1172, "2021-09-10"),
        ];
        assert_eq!(
            rolling_average_steps(rows),
            vec![
                (1, "2021-09-06".into(), 535.33),
                (1, "2021-09-07".into(), 595.67),
                (3, "2021-09-09".into(), 505.67),
                (3, "2021-09-10".into(), 674.67),
            ]
        );
    }

    #[test]
    fn requires_consecutive_calendar_days() {
        assert!(rolling_average_steps(vec![
            (1, 10, "2021-12-30"),
            (1, 20, "2021-12-31"),
            (1, 30, "2022-01-02"),
        ])
        .is_empty());
    }
}
