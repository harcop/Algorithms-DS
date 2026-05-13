/// LeetCode #720 - Longest Word in Dictionary
use std::collections::HashSet;

fn longest_word(words: Vec<String>) -> String {
    let mut words = words;
    words.sort();

    let mut built = HashSet::new();
    let mut answer = String::new();

    for word in words {
        if word.len() == 1 || built.contains(&word[..word.len() - 1]) {
            if word.len() > answer.len() {
                answer = word.clone();
            }

            built.insert(word);
        }
    }

    answer
}

fn main() {
    let words = vec![
        "w".to_string(),
        "wo".to_string(),
        "wor".to_string(),
        "worl".to_string(),
        "world".to_string(),
    ];

    println!("{}", longest_word(words));
}

#[cfg(test)]
mod tests {
    use super::longest_word;

    #[test]
    fn example_one() {
        let words = vec![
            "w".to_string(),
            "wo".to_string(),
            "wor".to_string(),
            "worl".to_string(),
            "world".to_string(),
        ];

        assert_eq!(longest_word(words), "world");
    }

    #[test]
    fn example_two() {
        let words = vec![
            "a".to_string(),
            "banana".to_string(),
            "app".to_string(),
            "appl".to_string(),
            "ap".to_string(),
            "apply".to_string(),
            "apple".to_string(),
        ];

        assert_eq!(longest_word(words), "apple");
    }
}