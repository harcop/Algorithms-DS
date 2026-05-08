/// LeetCode #500 - Keyboard Row
fn find_words(words: Vec<String>) -> Vec<String> {
    fn row(c: char) -> i32 {
        match c.to_ascii_lowercase() {
            'q' | 'w' | 'e' | 'r' | 't' | 'y' | 'u' | 'i' | 'o' | 'p' => 1,
            'a' | 's' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' => 2,
            _ => 3,
        }
    }
    let mut out = vec![];
    for w in words {
        let mut it = w.chars();
        let Some(first) = it.next() else {
            out.push(w);
            continue;
        };
        let r = row(first);
        if it.all(|c| row(c) == r) {
            out.push(w);
        }
    }
    out
}

fn main() {
    println!("{:?}", find_words(vec!["Hello".into(), "Alaska".into(), "Dad".into(), "Peace".into()]));
}

#[cfg(test)]
mod tests {
    use super::find_words;

    #[test]
    fn example_one() {
        assert_eq!(
            find_words(vec!["Hello".into(), "Alaska".into(), "Dad".into(), "Peace".into()]),
            vec!["Alaska".to_string(), "Dad".to_string()]
        );
    }
}
