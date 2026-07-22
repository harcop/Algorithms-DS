/// LeetCode #2586 - Count the Number of Vowel Strings in Range
fn is_vowel(c: u8) -> bool {
    matches!(c, b'a' | b'e' | b'i' | b'o' | b'u')
}

fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
    let mut ans = 0;
    for i in left..=right {
        let w = words[i as usize].as_bytes();
        if is_vowel(w[0]) && is_vowel(w[w.len() - 1]) {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        vowel_strings(
            vec!["are".into(), "amy".into(), "u".into()],
            0,
            2
        )
    );
}

#[cfg(test)]
mod tests {
    use super::vowel_strings;

    #[test]
    fn example_one() {
        assert_eq!(
            vowel_strings(vec!["are".into(), "amy".into(), "u".into()], 0, 2),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            vowel_strings(
                vec![
                    "hey".into(),
                    "aeo".into(),
                    "mu".into(),
                    "ooo".into(),
                    "artro".into()
                ],
                1,
                4
            ),
            3
        );
    }
}
