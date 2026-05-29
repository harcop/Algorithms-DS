/// LeetCode #1507 - Reformat Date
fn reformat_date(date: String) -> String {
    let parts: Vec<&str> = date.split_whitespace().collect();
    let day: i32 = parts[0].trim_end_matches(|c: char| !c.is_ascii_digit()).parse().unwrap();
    let month = match parts[1] {
        "Jan" => 1, "Feb" => 2, "Mar" => 3, "Apr" => 4, "May" => 5, "Jun" => 6,
        "Jul" => 7, "Aug" => 8, "Sep" => 9, "Oct" => 10, "Nov" => 11, _ => 12,
    };
    format!("{}-{:02}-{:02}", parts[2], month, day)
}
fn main() { println!("{}", reformat_date("20th Oct 2052".into())); }
#[cfg(test)]
mod tests {
    use super::reformat_date;
    #[test]
    fn example_one() { assert_eq!(reformat_date("20th Oct 2052".into()), "2052-10-20"); }
    #[test]
    fn example_two() { assert_eq!(reformat_date("6th Jun 1933".into()), "1933-06-06"); }
}
