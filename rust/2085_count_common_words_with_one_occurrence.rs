/// LeetCode #2085 - Count Common Words With One Occurrence
use std::collections::HashMap;

fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
    let count1 = frequencies(words1);
    let count2 = frequencies(words2);

    count1
        .iter()
        .filter(|(word, &count)| count == 1 && count2.get(*word) == Some(&1))
        .count() as i32
}

fn frequencies(words: Vec<String>) -> HashMap<String, i32> {
    let mut counts = HashMap::new();
    for word in words {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

fn main() {
    println!(
        "{}",
        count_words(
            vec!["leetcode".into(), "is".into(), "amazing".into(), "as".into(), "is".into()],
            vec!["amazing".into(), "leetcode".into(), "is".into()]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::count_words;

    fn strings(words: &[&str]) -> Vec<String> {
        words.iter().map(|word| (*word).to_string()).collect()
    }

    #[test]
    fn example_one() {
        assert_eq!(
            count_words(
                strings(&["leetcode", "is", "amazing", "as", "is"]),
                strings(&["amazing", "leetcode", "is"]),
            ),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_words(strings(&["b", "bb", "bbb"]), strings(&["a", "aa", "aaa"])),
            0
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(count_words(strings(&["a", "ab"]), strings(&["a", "a", "a", "ab"])), 1);
    }
}
