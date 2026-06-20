/// LeetCode #2000 - Reverse Prefix of Word
fn reverse_prefix(word: String, ch: String) -> String {
    let ch = ch.chars().next().unwrap();
    match word.find(ch) {
        None => word,
        Some(i) => {
            let mut prefix: String = word[..=i].chars().rev().collect();
            prefix.push_str(&word[i + 1..]);
            prefix
        }
    }
}

fn main() {
    println!("{}", reverse_prefix("abcdefd".into(), "d".into()));
}

#[cfg(test)]
mod tests {
    use super::reverse_prefix;

    #[test]
    fn example_one() {
        assert_eq!(reverse_prefix("abcdefd".into(), "d".into()), "dcbaefd");
    }

    #[test]
    fn example_two() {
        assert_eq!(reverse_prefix("xyxzxe".into(), "z".into()), "zxyxxe");
    }
}
