/// LeetCode #1078 - Occurrences After Bigram
fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut ans = Vec::new();
    for i in 0..words.len().saturating_sub(2) {
        if words[i] == first.as_str() && words[i + 1] == second.as_str() {
            ans.push(words[i + 2].to_string());
        }
    }
    ans
}

fn main() {
    println!("{:?}", find_ocurrences("alice is a good girl".into(), "a".into(), "good".into()));
}

#[cfg(test)]
mod tests {
    use super::find_ocurrences;

    #[test]
    fn example_one() {
        assert_eq!(
            find_ocurrences("alice is a good girl".into(), "a".into(), "good".into()),
            vec!["girl".to_string()]
        );
    }
}
