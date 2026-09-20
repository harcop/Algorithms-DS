/// LeetCode #3838 - Weighted Word Mapping
fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
    let mut ans = String::new();
    for w in words {
        let mut s = 0;
        for c in w.bytes() {
            s = (s + weights[(c - b'a') as usize]) % 26;
        }
        ans.push((b'a' + (25 - s) as u8) as char);
    }
    ans
}

fn main() {
    println!(
        "{}",
        map_word_weights(
            vec!["abcd".into(), "def".into(), "xyz".into()],
            vec![
                5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7, 2
            ]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::map_word_weights;

    #[test]
    fn example1() {
        assert_eq!(
            map_word_weights(
                vec!["abcd".into(), "def".into(), "xyz".into()],
                vec![
                    5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7,
                    7, 2
                ]
            ),
            "rij"
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            map_word_weights(
                vec!["a".into(), "b".into(), "c".into()],
                vec![1; 26]
            ),
            "yyy"
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            map_word_weights(
                vec!["abcd".into()],
                vec![
                    7, 5, 3, 4, 3, 5, 4, 9, 4, 2, 2, 7, 10, 2, 5, 10, 6, 1, 2, 2, 4, 1, 3, 4, 4,
                    5
                ]
            ),
            "g"
        );
    }
}
