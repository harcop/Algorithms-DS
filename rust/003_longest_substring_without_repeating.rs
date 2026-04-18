use std::collections::HashMap;

/// LeetCode #3 - Longest Substring Without Repeating Characters
///
/// Returns the length of the longest substring without repeating characters.
fn length_of_longest_substring(s: String) -> i32 {
    let mut last_index: HashMap<char, usize> = HashMap::new();
    let mut window_start = 0usize;
    let mut best = 0usize;

    for (i, c) in s.char_indices() {
        if let Some(&prev) = last_index.get(&c) {
            window_start = window_start.max(prev + 1);
        }
        last_index.insert(c, i);
        best = best.max(i - window_start + 1);
    }

    best as i32
}

fn main() {
    println!("{}", length_of_longest_substring("abcabcbb".to_string()));
}

#[cfg(test)]
mod tests {
    use super::length_of_longest_substring;

    #[test]
    fn example_one() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }
}
