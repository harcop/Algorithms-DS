/// LeetCode #2546 - Apply Bitwise Operations to Make Strings Equal
fn make_strings_equal(s: String, target: String) -> bool {
    s.contains('1') == target.contains('1')
}

fn main() {
    println!("{}", make_strings_equal("1010".to_string(), "0110".to_string()));
}

#[cfg(test)]
mod tests {
    use super::make_strings_equal;

    #[test]
    fn example_one() {
        assert!(make_strings_equal("1010".to_string(), "0110".to_string()));
    }

    #[test]
    fn example_two() {
        assert!(!make_strings_equal("11".to_string(), "00".to_string()));
    }
}
