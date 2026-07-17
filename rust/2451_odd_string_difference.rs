/// LeetCode #2451 - Odd String Difference
fn odd_string(words: Vec<String>) -> String {
    fn difference(word: &str) -> Vec<i32> {
        word.as_bytes()
            .windows(2)
            .map(|pair| pair[1] as i32 - pair[0] as i32)
            .collect()
    }

    let first = difference(&words[0]);
    let second = difference(&words[1]);
    let common = if first == second {
        first
    } else {
        let third = difference(&words[2]);
        if first == third {
            first
        } else {
            second
        }
    };

    words
        .into_iter()
        .find(|word| difference(word) != common)
        .unwrap()
}

fn main() {
    println!(
        "{}",
        odd_string(vec![
            "adc".to_string(),
            "wzy".to_string(),
            "abc".to_string()
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::odd_string;

    #[test]
    fn example_one() {
        assert_eq!(
            odd_string(vec![
                "adc".to_string(),
                "wzy".to_string(),
                "abc".to_string()
            ]),
            "abc"
        );
    }

    #[test]
    fn odd_word_comes_first() {
        assert_eq!(
            odd_string(vec![
                "aaa".to_string(),
                "bob".to_string(),
                "ccc".to_string()
            ]),
            "bob"
        );
    }
}
