/// LeetCode #2135 - Count Words Obtained After Adding a Letter
use std::collections::HashSet;

fn word_count(start_words: Vec<String>, target_words: Vec<String>) -> i32 {
    let starts: HashSet<i32> = start_words.into_iter().map(mask).collect();
    let mut count = 0;

    for word in target_words {
        let target = mask(word);
        for bit in 0..26 {
            if target & (1 << bit) != 0 && starts.contains(&(target ^ (1 << bit))) {
                count += 1;
                break;
            }
        }
    }

    count
}

fn mask(word: String) -> i32 {
    word.bytes()
        .fold(0, |bits, b| bits | (1 << (b - b'a') as i32))
}

fn main() {
    println!(
        "{}",
        word_count(
            vec!["ant".into(), "act".into(), "tack".into()],
            vec!["tack".into(), "act".into(), "acti".into()]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::word_count;

    #[test]
    fn example_one() {
        assert_eq!(
            word_count(
                vec!["ant".into(), "act".into(), "tack".into()],
                vec!["tack".into(), "act".into(), "acti".into()]
            ),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            word_count(
                vec!["ab".into(), "a".into()],
                vec!["abc".into(), "abcd".into()]
            ),
            1
        );
    }
}
