/// LeetCode #1185 - Day of the Week
fn day_of_the_week(day: i32, mut month: i32, mut year: i32) -> String {
    const DAYS: [&str; 7] = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    if month < 3 {
        month += 12;
        year -= 1;
    }
    let c = year / 100;
    let y = year % 100;
    let w = (c / 4 - 2 * c + y + y / 4 + (13 * (month + 1)) / 5 + day - 1).rem_euclid(7);
    DAYS[w as usize].to_string()
}

fn main() {
    println!("{}", day_of_the_week(31, 8, 2019));
}

#[cfg(test)]
mod tests {
    use super::day_of_the_week;

    #[test]
    fn example_one() {
        assert_eq!(day_of_the_week(31, 8, 2019), "Saturday");
    }

    #[test]
    fn example_two() {
        assert_eq!(day_of_the_week(18, 7, 1999), "Sunday");
    }

    #[test]
    fn example_three() {
        assert_eq!(day_of_the_week(15, 8, 1993), "Sunday");
    }
}
