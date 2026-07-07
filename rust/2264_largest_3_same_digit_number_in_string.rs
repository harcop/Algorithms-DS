/// LeetCode #2264 - Largest 3-Same-Digit Number in String
fn largest_good_integer(num: String) -> String {
    for d in (b'0'..=b'9').rev() {
        let s = String::from_utf8(vec![d, d, d]).unwrap();
        if num.contains(&s) {
            return s;
        }
    }
    String::new()
}

fn main() {
    println!("{}", largest_good_integer("6777133339".to_string()));
}

#[cfg(test)]
mod tests {
    use super::largest_good_integer;

    #[test]
    fn example_one() {
        assert_eq!(largest_good_integer("6777133339".to_string()), "777");
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_good_integer("2300019".to_string()), "000");
    }

    #[test]
    fn example_three() {
        assert_eq!(largest_good_integer("42352338".to_string()), "");
    }
}
