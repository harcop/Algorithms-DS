/// LeetCode #804 - Unique Morse Code Words
use std::collections::HashSet;

fn unique_morse_representations(words: Vec<String>) -> i32 {
    const CODE: [&str; 26] = [
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
    ];
    let mut set = HashSet::new();
    for w in words {
        let mut s = String::new();
        for c in w.bytes() {
            s.push_str(CODE[(c - b'a') as usize]);
        }
        set.insert(s);
    }
    set.len() as i32
}

fn main() {
    println!(
        "{}",
        unique_morse_representations(vec!["gin".into(), "zen".into(), "gig".into(), "msg".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::unique_morse_representations;

    #[test]
    fn example_one() {
        assert_eq!(
            unique_morse_representations(vec![
                "gin".into(),
                "zen".into(),
                "gig".into(),
                "msg".into()
            ]),
            2
        );
    }
}
