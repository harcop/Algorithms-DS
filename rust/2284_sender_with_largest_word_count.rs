/// LeetCode #2284 - Sender With Largest Word Count
use std::collections::HashMap;

fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
    let mut counts: HashMap<String, i32> = HashMap::new();
    for (msg, sender) in messages.into_iter().zip(senders.into_iter()) {
        let words = msg.split_whitespace().count() as i32;
        *counts.entry(sender).or_insert(0) += words;
    }

    let mut best_sender = String::new();
    let mut best_count = -1;
    for (sender, count) in counts {
        if count > best_count || (count == best_count && sender > best_sender) {
            best_count = count;
            best_sender = sender;
        }
    }
    best_sender
}

fn main() {
    println!(
        "{}",
        largest_word_count(
            vec!["Hello world".to_string()],
            vec!["Alice".to_string()]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::largest_word_count;

    #[test]
    fn example_one() {
        let messages = vec![
            "Hello userTwooo".to_string(),
            "Hi userThree".to_string(),
            "Wonderful day Alice".to_string(),
            "Nice day userThree".to_string(),
        ];
        let senders = vec![
            "Alice".to_string(),
            "userTwo".to_string(),
            "userThree".to_string(),
            "Alice".to_string(),
        ];
        assert_eq!(largest_word_count(messages, senders), "userThree".to_string());
    }
}

