/// LeetCode #1903 - Largest Odd Number in String
fn largest_odd_number(num: String) -> String {
    let bytes = num.as_bytes();
    for i in (0..bytes.len()).rev() {
        if (bytes[i] - b'0') & 1 == 1 {
            return num[..=i].to_string();
        }
    }
    String::new()
}

fn main() {
    println!("{}", largest_odd_number("52".into()));
}

#[cfg(test)]
mod tests {
    use super::largest_odd_number;

    #[test]
    fn example_one() {
        assert_eq!(largest_odd_number("52".into()), "5");
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_odd_number("4206".into()), "");
    }

    #[test]
    fn example_three() {
        assert_eq!(largest_odd_number("35427".into()), "35427");
    }
}
