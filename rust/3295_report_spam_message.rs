/// LeetCode #3295 - Report Spam Message
use std::collections::HashSet;

fn report_spam(message: Vec<String>, banned_words: Vec<String>) -> bool {
    let s: HashSet<_> = banned_words.into_iter().collect();
    message.iter().filter(|w| s.contains(*w)).count() >= 2
}

fn main() {
    println!(
        "{}",
        report_spam(
            vec!["hello".into(), "world".into(), "leetcode".into()],
            vec!["world".into(), "hello".into()]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::report_spam;

    #[test]
    fn example1() {
        assert!(report_spam(
            vec!["hello".into(), "world".into(), "leetcode".into()],
            vec!["world".into(), "hello".into()]
        ));
    }

    #[test]
    fn example2() {
        assert!(!report_spam(
            vec!["hello".into(), "programming".into(), "fun".into()],
            vec!["world".into(), "programming".into(), "leetcode".into()]
        ));
    }
}
