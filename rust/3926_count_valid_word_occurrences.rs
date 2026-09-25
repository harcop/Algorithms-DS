/// LeetCode #3926 - Count Valid Word Occurrences
use std::collections::HashMap;

fn count_word_occurrences(chunks: Vec<String>, queries: Vec<String>) -> Vec<i32> {
    let s = chunks.concat();
    let b = s.as_bytes();
    let n = b.len();
    let mut cnt: HashMap<&str, i32> = HashMap::new();
    let mut i = 0;
    while i < n {
        if b[i] == b' ' || b[i] == b'-' {
            i += 1;
            continue;
        }
        let mut j = i;
        while j < n
            && b[j] != b' '
            && (b[j] != b'-' || (j + 1 < n && b[j + 1] != b' ' && b[j + 1] != b'-'))
        {
            j += 1;
        }
        *cnt.entry(&s[i..j]).or_insert(0) += 1;
        i = j;
    }
    queries
        .iter()
        .map(|q| cnt.get(q.as_str()).copied().unwrap_or(0))
        .collect()
}

fn main() {
    println!(
        "{:?}",
        count_word_occurrences(
            vec!["hello wor".into(), "ld hello".into()],
            vec!["hello".into(), "world".into(), "wor".into()]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::count_word_occurrences;

    #[test]
    fn example1() {
        assert_eq!(
            count_word_occurrences(
                vec!["hello wor".into(), "ld hello".into()],
                vec!["hello".into(), "world".into(), "wor".into()]
            ),
            vec![2, 1, 0]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_word_occurrences(
                vec!["a--b a-".into(), "-c".into()],
                vec!["a".into(), "b".into(), "c".into()]
            ),
            vec![2, 1, 1]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            count_word_occurrences(vec!["hello".into()], vec!["hello".into(), "ell".into()]),
            vec![1, 0]
        );
    }
}
