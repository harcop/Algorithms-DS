use std::collections::HashMap;

/// LeetCode #30 - Substring with Concatenation of All Words
fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    if words.is_empty() {
        return vec![];
    }

    let word_len = words[0].len();
    let word_count = words.len();
    let total_len = word_len * word_count;
    if s.len() < total_len {
        return vec![];
    }

    let mut target: HashMap<&str, i32> = HashMap::new();
    for w in &words {
        *target.entry(w.as_str()).or_insert(0) += 1;
    }

    let mut result = Vec::new();
    for offset in 0..word_len {
        let mut left = offset;
        let mut seen: HashMap<&str, i32> = HashMap::new();
        let mut used = 0usize;
        let mut right = offset;

        while right + word_len <= s.len() {
            let word = &s[right..right + word_len];
            right += word_len;

            if target.contains_key(word) {
                *seen.entry(word).or_insert(0) += 1;
                used += 1;

                while seen[word] > target[word] {
                    let left_word = &s[left..left + word_len];
                    if let Some(count) = seen.get_mut(left_word) {
                        *count -= 1;
                    }
                    left += word_len;
                    used -= 1;
                }

                if used == word_count {
                    result.push(left as i32);
                    let left_word = &s[left..left + word_len];
                    if let Some(count) = seen.get_mut(left_word) {
                        *count -= 1;
                    }
                    left += word_len;
                    used -= 1;
                }
            } else {
                seen.clear();
                used = 0;
                left = right;
            }
        }
    }

    result
}

fn main() {
    let out = find_substring(
        "barfoothefoobarman".to_string(),
        vec!["foo".to_string(), "bar".to_string()],
    );
    println!("{out:?}");
}

#[cfg(test)]
mod tests {
    use super::find_substring;

    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort_unstable();
        v
    }

    #[test]
    fn example_one() {
        let out = find_substring(
            "barfoothefoobarman".to_string(),
            vec!["foo".to_string(), "bar".to_string()],
        );
        assert_eq!(sorted(out), vec![0, 9]);
    }

    #[test]
    fn example_two() {
        let out = find_substring(
            "wordgoodgoodgoodbestword".to_string(),
            vec![
                "word".to_string(),
                "good".to_string(),
                "best".to_string(),
                "word".to_string(),
            ],
        );
        assert!(out.is_empty());
    }

    #[test]
    fn example_three() {
        let out = find_substring(
            "barfoofoobarthefoobarman".to_string(),
            vec!["bar".to_string(), "foo".to_string(), "the".to_string()],
        );
        assert_eq!(sorted(out), vec![6, 9, 12]);
    }
}
