/// LeetCode #2063 - Vowels of All Substrings
fn count_vowels(word: String) -> i64 {
    let n = word.len();
    word.bytes()
        .enumerate()
        .filter(|(_, c)| matches!(c, b'a' | b'e' | b'i' | b'o' | b'u'))
        .map(|(i, _)| ((i + 1) * (n - i)) as i64)
        .sum()
}

fn main() {
    println!("{}", count_vowels("aba".into()));
}

#[cfg(test)]
mod tests {
    use super::count_vowels;

    #[test]
    fn example_one() {
        assert_eq!(count_vowels("aba".into()), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_vowels("abc".into()), 3);
    }
}
