/// LeetCode #125 - Valid Palindrome
fn is_palindrome(s: String) -> bool {
    let b: Vec<u8> = s.bytes().filter(|&c| c.is_ascii_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect();
    let mut l = 0usize;
    let mut r = b.len().saturating_sub(1);
    while l < r {
        if b[l] != b[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}

fn main() {
    println!("{}", is_palindrome("A man, a plan, a canal: Panama".to_string()));
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn example_one() {
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
    }

    #[test]
    fn example_two() {
        assert!(!is_palindrome("race a car".to_string()));
    }

    #[test]
    fn example_three() {
        assert!(is_palindrome(" ".to_string()));
    }
}
