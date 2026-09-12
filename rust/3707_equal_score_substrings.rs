/// LeetCode #3707 - Equal Score Substrings
fn score_balance(s: String) -> bool {
    let total: i32 = s.bytes().map(|c| (c - b'a' + 1) as i32).sum();
    let mut prefix = 0;
    for c in s.bytes() {
        prefix += (c - b'a' + 1) as i32;
        if prefix == total - prefix {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", score_balance("adcb".into()));
}

#[cfg(test)]
mod tests {
    use super::score_balance;

    #[test]
    fn example1() {
        assert!(score_balance("adcb".into()));
    }

    #[test]
    fn example2() {
        assert!(!score_balance("bace".into()));
    }
}
