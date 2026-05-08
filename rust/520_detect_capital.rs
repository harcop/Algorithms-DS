/// LeetCode #520 - Detect Capital
fn detect_capital_use(word: String) -> bool {
    let bytes: Vec<u8> = word.bytes().collect();
    if bytes.len() <= 1 {
        return true;
    }
    let is_upper = |b: u8| (b'A'..=b'Z').contains(&b);
    let all_upper = bytes.iter().all(|&b| is_upper(b));
    let all_lower = bytes.iter().all(|&b| !is_upper(b));
    let first_upper_rest_lower = is_upper(bytes[0]) && bytes[1..].iter().all(|&b| !is_upper(b));
    all_upper || all_lower || first_upper_rest_lower
}

fn main() {
    println!("{}", detect_capital_use("USA".into()));
}

#[cfg(test)]
mod tests {
    use super::detect_capital_use;

    #[test]
    fn example_one() {
        assert!(detect_capital_use("USA".into()));
    }

    #[test]
    fn example_two() {
        assert!(detect_capital_use("Google".into()));
    }

    #[test]
    fn example_three() {
        assert!(!detect_capital_use("FlaG".into()));
    }
}
