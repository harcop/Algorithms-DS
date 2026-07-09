/// LeetCode #2330 - Valid Palindrome IV
fn make_palindrome(s: String) -> bool {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    let mut j = bytes.len() - 1;
    let mut cnt = 0;
    while i < j {
        cnt += (bytes[i] != bytes[j]) as i32;
        i += 1;
        j -= 1;
    }
    cnt <= 2
}

fn main() {
    println!("{}", make_palindrome("abcdba".to_string()));
}

#[cfg(test)]
mod tests {
    use super::make_palindrome;

    #[test]
    fn example_one() {
        assert!(make_palindrome("abcdba".to_string()));
    }

    #[test]
    fn example_two() {
        assert!(make_palindrome("aa".to_string()));
    }

    #[test]
    fn example_three() {
        assert!(!make_palindrome("abcdef".to_string()));
    }
}
