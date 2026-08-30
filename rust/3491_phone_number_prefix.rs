/// LeetCode #3491 - Phone Number Prefix
fn phone_prefix(mut numbers: Vec<String>) -> bool {
    numbers.sort_by_key(|s| s.len());
    for (i, s) in numbers.iter().enumerate() {
        if numbers[..i].iter().any(|t| s.starts_with(t)) {
            return false;
        }
    }
    true
}

fn main() {
    println!(
        "{}",
        phone_prefix(vec!["1".into(), "2".into(), "4".into(), "3".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::phone_prefix;

    #[test]
    fn example1() {
        assert!(phone_prefix(vec![
            "1".into(),
            "2".into(),
            "4".into(),
            "3".into()
        ]));
    }

    #[test]
    fn example2() {
        assert!(!phone_prefix(vec![
            "001".into(),
            "007".into(),
            "15".into(),
            "00153".into()
        ]));
    }
}
