/// LeetCode #58 - Length of Last Word
fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace().last().map_or(0, |w| w.len() as i32)
}

fn main() {
    println!("{}", length_of_last_word("Hello World".to_string()));
}

#[cfg(test)]
mod tests {
    use super::length_of_last_word;
    #[test]
    fn example_one() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
    }
    #[test]
    fn example_two() {
        assert_eq!(length_of_last_word("   fly me   to   the moon  ".to_string()), 4);
    }
}
