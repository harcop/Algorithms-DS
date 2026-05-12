/// LeetCode #680 - Valid Palindrome II
fn valid_palindrome(s: String) -> bool {
    let b = s.as_bytes();
    let mut l = 0usize;
    let mut r = b.len() - 1;
    while l < r {
        if b[l] != b[r] {
            return is_pal(b, l + 1, r) || is_pal(b, l, r - 1);
        }
        l += 1;
        r -= 1;
    }
    true
}

fn is_pal(b: &[u8], mut l: usize, mut r: usize) -> bool {
    while l < r {
        if b[l] != b[r] { return false; }
        l += 1; r -= 1;
    }
    true
}

fn main() {
    println!("{}", valid_palindrome("abca".into()));
}

#[cfg(test)]
mod tests {
    use super::valid_palindrome;

    #[test]
    fn example_one() {
        assert!(valid_palindrome("aba".into()));
    }

    #[test]
    fn example_two() {
        assert!(valid_palindrome("abca".into()));
    }

    #[test]
    fn example_three() {
        assert!(!valid_palindrome("abc".into()));
    }
}
