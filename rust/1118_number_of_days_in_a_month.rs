/// LeetCode #1118 - Number of Days in a Month
fn number_of_days(year: i32, month: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let leap = year % 400 == 0 || (year % 4 == 0 && year % 100 != 0);
            if leap { 29 } else { 28 }
        }
        _ => 0,
    }
}

fn main() {
    println!("{}", number_of_days(1992, 7));
}

#[cfg(test)]
mod tests {
    use super::number_of_days;

    #[test]
    fn example_one() {
        assert_eq!(number_of_days(1992, 7), 31);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_days(2000, 2), 29);
    }

    #[test]
    fn example_three() {
        assert_eq!(number_of_days(1900, 2), 28);
    }
}
