/// LeetCode #1853 - Convert Date Format (SQL; Rust analogue)
fn weekday(y: i32, m: i32, d: i32) -> i32 {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if m < 3 { y - 1 } else { y };
    (y + y / 4 - y / 100 + y / 400 + t[(m - 1) as usize] + d) % 7
}

fn convert_date_format(days: Vec<String>) -> Vec<String> {
    const WEEKDAYS: [&str; 7] = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    const MONTHS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    days.into_iter()
        .map(|s| {
            let mut p = s.split('-');
            let y: i32 = p.next().unwrap().parse().unwrap();
            let m: i32 = p.next().unwrap().parse().unwrap();
            let d: i32 = p.next().unwrap().parse().unwrap();
            format!(
                "{}, {} {}, {}",
                WEEKDAYS[weekday(y, m, d) as usize],
                MONTHS[(m - 1) as usize],
                d,
                y
            )
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        convert_date_format(vec![
            "2022-04-12".into(),
            "2021-08-09".into(),
            "2020-06-26".into()
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::convert_date_format;

    #[test]
    fn example_one() {
        let days = vec![
            "2022-04-12".into(),
            "2021-08-09".into(),
            "2020-06-26".into(),
        ];
        assert_eq!(
            convert_date_format(days),
            vec![
                "Tuesday, April 12, 2022".to_string(),
                "Monday, August 9, 2021".to_string(),
                "Friday, June 26, 2020".to_string(),
            ]
        );
    }
}
