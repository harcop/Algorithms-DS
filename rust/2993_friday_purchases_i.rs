/// LeetCode #2993 - Friday Purchases I (SQL; Rust analogue)
use std::collections::HashMap;

const FRIDAY_DATES: &[(&str, i32)] = &[
    ("2023-11-03", 1),
    ("2023-11-10", 2),
    ("2023-11-17", 3),
    ("2023-11-24", 4),
];

fn friday_purchases_i(purchases: Vec<(i32, String, i32)>) -> Vec<(i32, String, i32)> {
    let fridays: HashMap<&str, i32> = FRIDAY_DATES.iter().copied().collect();
    let mut totals: HashMap<String, i32> = HashMap::new();
    for (_, date, amount) in purchases {
        if fridays.contains_key(date.as_str()) {
            *totals.entry(date).or_default() += amount;
        }
    }
    let mut ans: Vec<_> = totals
        .into_iter()
        .map(|(date, total)| (fridays[date.as_str()], date, total))
        .collect();
    ans.sort_by_key(|(week, _, _)| *week);
    ans
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
    println!("{:?}", friday_purchases_i(purchases));
}

#[cfg(test)]
mod tests {
    use super::friday_purchases_i;

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
            friday_purchases_i(purchases),
            vec![
                (1, "2023-11-03".into(), 5117),
                (4, "2023-11-24".into(), 21692),
            ]
        );
    }
}
