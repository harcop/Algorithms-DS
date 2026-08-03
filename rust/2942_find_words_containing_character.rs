/// LeetCode #2942 - Find Words Containing Character
fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    words
        .iter()
        .enumerate()
        .filter_map(|(i, w)| if w.contains(x) { Some(i as i32) } else { None })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        find_words_containing(vec!["leet".into(), "code".into()], 'e')
    );
}

#[cfg(test)]
mod tests {
    use super::find_words_containing;

    #[test]
    fn example_one() {
        assert_eq!(
            find_words_containing(vec!["leet".into(), "code".into()], 'e'),
            vec![0, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_words_containing(
                vec!["abc".into(), "bcd".into(), "aaaa".into(), "cbc".into()],
                'a'
            ),
            vec![0, 2]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            find_words_containing(
                vec!["abc".into(), "bcd".into(), "aaaa".into(), "cbc".into()],
                'z'
            ),
            Vec::<i32>::new()
        );
    }
}
