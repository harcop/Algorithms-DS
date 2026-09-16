/// LeetCode #3775 - Reverse Words With Same Vowel Count
fn reverse_words(s: String) -> String {
    fn vowels(w: &str) -> usize {
        w.bytes()
            .filter(|&c| matches!(c, b'a' | b'e' | b'i' | b'o' | b'u'))
            .count()
    }
    let mut words: Vec<String> = s.split_whitespace().map(|w| w.to_string()).collect();
    let cnt = vowels(&words[0]);
    for w in words.iter_mut().skip(1) {
        if vowels(w) == cnt {
            let rev: String = w.chars().rev().collect();
            *w = rev;
        }
    }
    words.join(" ")
}

fn main() {
    println!("{}", reverse_words("cat and mice".into()));
}

#[cfg(test)]
mod tests {
    use super::reverse_words;

    #[test]
    fn example1() {
        assert_eq!(reverse_words("cat and mice".into()), "cat dna mice");
    }

    #[test]
    fn example2() {
        assert_eq!(reverse_words("book is nice".into()), "book is ecin");
    }

    #[test]
    fn example3() {
        assert_eq!(reverse_words("banana healthy".into()), "banana healthy");
    }
}
