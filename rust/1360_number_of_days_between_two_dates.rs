/// LeetCode #1360 - Number Of Days Between Two Dates

fn is_leap(y: i32) -> bool {
    y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
}

fn days_in_month(year: i32, month: i32) -> i32 {
    const DIM: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut d = DIM[(month - 1) as usize];
    if month == 2 && is_leap(year) {
        d += 1;
    }
    d
}

fn day_number(year: i32, month: i32, day: i32) -> i32 {
    let mut days = day;
    for m in 1..month {
        days += days_in_month(year, m);
    }
    for y in 1971..year {
        days += if is_leap(y) { 366 } else { 365 };
    }
    days
}

fn days_between_dates(date1: String, date2: String) -> i32 {
    fn parse(s: &str) -> (i32, i32, i32) {
        let p: Vec<i32> = s.split('-').map(|x| x.parse().unwrap()).collect();
        (p[0], p[1], p[2])
    }
    let d1 = parse(&date1);
    let d2 = parse(&date2);
    (day_number(d2.0, d2.1, d2.2) - day_number(d1.0, d1.1, d1.2)).abs()
}

fn main() {
    println!("{}", days_between_dates("2019-06-29".into(), "2019-06-30".into()));
}

#[cfg(test)]
mod tests {
    use super::days_between_dates;

    #[test]
    fn example_one() {
        assert_eq!(days_between_dates("2019-06-29".into(), "2019-06-30".into()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(days_between_dates("2020-01-15".into(), "2019-12-31".into()), 15);
    }
}
