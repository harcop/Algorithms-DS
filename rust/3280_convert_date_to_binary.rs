/// LeetCode #3280 - Convert Date to Binary
fn convert_date_to_binary(date: String) -> String {
    date.split('-')
        .map(|s| format!("{:b}", s.parse::<i32>().unwrap()))
        .collect::<Vec<_>>()
        .join("-")
}

fn main() {
    println!("{}", convert_date_to_binary("2080-02-29".into()));
}

#[cfg(test)]
mod tests {
    use super::convert_date_to_binary;

    #[test]
    fn example1() {
        assert_eq!(
            convert_date_to_binary("2080-02-29".into()),
            "100000100000-10-11101"
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            convert_date_to_binary("1900-01-01".into()),
            "11101101100-1-1"
        );
    }
}
