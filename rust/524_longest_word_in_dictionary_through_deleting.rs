/// LeetCode #524 - Longest Word in Dictionary through Deleting
fn is_subseq(word: &str, s: &str) -> bool {
    let mut it = s.chars();
    word.chars().all(|c| it.any(|d| d == c))
}

fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
    let mut best = String::new();
    for w in dictionary {
        if is_subseq(&w, &s)
            && (w.len() > best.len() || (w.len() == best.len() && w < best))
        {
            best = w;
        }
    }
    best
}

fn main() {
    let s = "abpcplea".to_string();
    let dictionary = vec!["ale".into(), "apple".into(), "monkey".into(), "plea".into()];
    println!("{}", find_longest_word(s, dictionary));
}

#[cfg(test)]
mod tests {
    use super::find_longest_word;

    #[test]
    fn example_one() {
        let s = "abpcplea".to_string();
        let dictionary = vec!["ale".into(), "apple".into(), "monkey".into(), "plea".into()];
        assert_eq!(find_longest_word(s, dictionary), "apple");
    }

    #[test]
    fn example_two() {
        let s = "abpcplea".to_string();
        let dictionary = vec!["a".into(), "b".into(), "c".into()];
        assert_eq!(find_longest_word(s, dictionary), "a");
    }
}
