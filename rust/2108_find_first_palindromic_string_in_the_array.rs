/// LeetCode #2108 - Find First Palindromic String in the Array
fn first_palindrome(words: Vec<String>) -> String {
    for word in words {
        if word.bytes().eq(word.bytes().rev()) {
            return word;
        }
    }
    String::new()
}

fn main() {
    println!("{}", first_palindrome(vec!["abc".into(), "car".into(), "ada".into()]));
}

#[cfg(test)]
mod tests {
    use super::first_palindrome;

    #[test]
    fn example_one() {
        assert_eq!(
            first_palindrome(vec!["abc".into(), "car".into(), "ada".into(), "racecar".into()]),
            "ada"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(first_palindrome(vec!["notapalindrome".into(), "racecar".into()]), "racecar");
    }

    #[test]
    fn example_three() {
        assert_eq!(first_palindrome(vec!["def".into(), "ghi".into()]), "");
    }
}
