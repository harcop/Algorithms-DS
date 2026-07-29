/// LeetCode #2758 - Next Day (JS problem; Rust analogue)
/// Given a date string "YYYY-MM-DD", return the next day as the same format.
fn next_day(date: &str) -> String {
    let parts: Vec<u32> = date.split('-').map(|s| s.parse().unwrap()).collect();
    let (mut y, mut m, mut d) = (parts[0], parts[1], parts[2]);
    let days_in_month = [0u32, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let is_leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let dim = if m == 2 && is_leap {
        29
    } else {
        days_in_month[m as usize]
    };
    d += 1;
    if d > dim {
        d = 1;
        m += 1;
        if m > 12 {
            m = 1;
            y += 1;
        }
    }
    format!("{:04}-{:02}-{:02}", y, m, d)
}

fn main() {
    println!("{}", next_day("2023-12-31"));
}

#[cfg(test)]
mod tests {
    use super::next_day;

    #[test]
    fn example_normal() {
        assert_eq!(next_day("2023-03-15"), "2023-03-16");
    }

    #[test]
    fn example_end_of_month() {
        assert_eq!(next_day("2023-03-31"), "2023-04-01");
    }

    #[test]
    fn example_end_of_year() {
        assert_eq!(next_day("2023-12-31"), "2024-01-01");
    }

    #[test]
    fn example_leap_year() {
        assert_eq!(next_day("2024-02-28"), "2024-02-29");
    }

    #[test]
    fn example_non_leap_year() {
        assert_eq!(next_day("2023-02-28"), "2023-03-01");
    }
}
