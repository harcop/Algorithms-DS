/// LeetCode #820 - Short Encoding of Words
use std::collections::HashSet;

fn minimum_length_encoding(words: Vec<String>) -> i32 {
    let mut set: HashSet<String> = words.into_iter().collect();
    for w in set.clone() {
        for i in 1..w.len() {
            let suf = &w[i..];
            set.remove(suf);
        }
    }
    set.iter().map(|w| w.len() + 1).sum::<usize>() as i32
}

fn main() {
    println!(
        "{}",
        minimum_length_encoding(vec!["time".into(), "me".into(), "bell".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_length_encoding;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_length_encoding(vec!["time".into(), "me".into(), "bell".into()]),
            10
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_length_encoding(vec!["t".into()]), 2);
    }
}
