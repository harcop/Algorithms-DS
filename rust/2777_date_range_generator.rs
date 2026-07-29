/// LeetCode #2777 - Date Range Generator (JS problem; Rust analogue)
fn date_range_generator(start: &str, end: &str, step: u32) -> Vec<String> {
    let mut dates = Vec::new();
    let mut current = parse_date(start);
    let end_date = parse_date(end);
    while current <= end_date {
        dates.push(format_date(current));
        current = add_days(current, step);
    }
    dates
}

fn parse_date(s: &str) -> (u32, u32, u32) {
    let parts: Vec<u32> = s.split('-').map(|p| p.parse().unwrap()).collect();
    (parts[0], parts[1], parts[2])
}

fn format_date((y, m, d): (u32, u32, u32)) -> String {
    format!("{:04}-{:02}-{:02}", y, m, d)
}

fn add_days((mut y, mut m, mut d): (u32, u32, u32), step: u32) -> (u32, u32, u32) {
    for _ in 0..step {
        d += 1;
        let dim = days_in_month(y, m);
        if d > dim {
            d = 1;
            m += 1;
            if m > 12 {
                m = 1;
                y += 1;
            }
        }
    }
    (y, m, d)
}

fn days_in_month(y: u32, m: u32) -> u32 {
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    match m {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn main() {
    println!("{:?}", date_range_generator("2023-04-01", "2023-04-04", 1));
}

#[cfg(test)]
mod tests {
    use super::date_range_generator;

    #[test]
    fn example_one() {
        assert_eq!(
            date_range_generator("2023-04-01", "2023-04-04", 1),
            vec![
                "2023-04-01".to_string(),
                "2023-04-02".to_string(),
                "2023-04-03".to_string(),
                "2023-04-04".to_string(),
            ]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            date_range_generator("2023-04-10", "2023-04-20", 3),
            vec![
                "2023-04-10".to_string(),
                "2023-04-13".to_string(),
                "2023-04-16".to_string(),
                "2023-04-19".to_string(),
            ]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            date_range_generator("2023-04-10", "2023-04-10", 1),
            vec!["2023-04-10".to_string()]
        );
    }
}
