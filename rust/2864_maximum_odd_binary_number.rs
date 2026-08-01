/// LeetCode #2864 - Maximum Odd Binary Number
fn maximum_odd_binary_number(s: String) -> String {
    let ones = s.chars().filter(|&c| c == '1').count();
    "1".repeat(ones - 1) + &"0".repeat(s.len() - ones) + "1"
}

fn main() {
    println!("{}", maximum_odd_binary_number("0101".to_string()));
}

#[cfg(test)]
mod tests {
    use super::maximum_odd_binary_number;

    #[test]
    fn example_one() {
        assert_eq!(maximum_odd_binary_number("010".to_string()), "001");
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_odd_binary_number("0101".to_string()), "1001");
    }
}
