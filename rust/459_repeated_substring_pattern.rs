/// LeetCode #459 - Repeated Substring Pattern
fn repeated_substring_pattern(s: String) -> bool {
    let n = s.len();
    if n < 2 {
        return false;
    }
    let mut doubled = s.clone();
    doubled.push_str(&s);
    doubled.as_bytes()[1..2 * n - 1]
        .windows(n)
        .any(|w| w == s.as_bytes())
}

fn main() {
    println!("{}", repeated_substring_pattern("abab".into()));
}

#[cfg(test)]
mod tests {
    use super::repeated_substring_pattern;

    #[test]
    fn example_one() {
        assert!(repeated_substring_pattern("abab".into()));
    }

    #[test]
    fn example_two() {
        assert!(!repeated_substring_pattern("aba".into()));
    }
}
