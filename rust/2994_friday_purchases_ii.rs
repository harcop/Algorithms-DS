/// LeetCode #2994 - Friday Purchases II (SQL; Rust analogue)
use std::collections::HashMap;

const FRIDAY_DATES: &[(&str, i32)] = &[
    ("2023-11-03", 1),
    ("2023-11-10", 2),
    ("2023-11-17", 3),
    ("2023-11-24", 4),
];

fn friday_purchases_ii(purchases: Vec<(i32, String, i32)>) -> Vec<(i32, String, i32)> {
    let mut totals: HashMap<String, i32> = HashMap::new();
    for (_, date, amount) in purchases {
        if FRIDAY_DATES.iter().any(|(d, _)| *d == date.as_str()) {
            *totals.entry(date).or_default() += amount;
        }
    }
    FRIDAY_DATES
        .iter()
        .map(|(date, week)| (*week, date.to_string(), *totals.get(*date).unwrap_or(&0)))
        .collect()
}

fn main() {
    let purchases = vec![
        (11, "2023-11-07".into(), 1126),
        (15, "2023-11-30".into(), 7473),
        (17, "2023-11-14".into(), 2414),
        (12, "2023-11-24".into(), 9692),
        (8, "2023-11-03".into(), 5117),
        (1, "2023-11-16".into(), 5241),
        (10, "2023-11-12".into(), 8266),
        (13, "2023-11-24".into(), 12000),
    ];
    println!("{:?}", friday_purchases_ii(purchases));
}

#[cfg(test)]
mod tests {
    use super::friday_purchases_ii;

    #[test]
    fn example() {
        let purchases = vec![
            (11, "2023-11-07".into(), 1126),
            (15, "2023-11-30".into(), 7473),
            (17, "2023-11-14".into(), 2414),
            (12, "2023-11-24".into(), 9692),
            (8, "2023-11-03".into(), 5117),
            (1, "2023-11-16".into(), 5241),
            (10, "2023-11-12".into(), 8266),
            (13, "2023-11-24".into(), 12000),
        ];
        assert_eq!(
            friday_purchases_ii(purchases),
            vec![
                (1, "2023-11-03".into(), 5117),
                (2, "2023-11-10".into(), 0),
                (3, "2023-11-17".into(), 0),
                (4, "2023-11-24".into(), 21692),
            ]
        );
    }
}
