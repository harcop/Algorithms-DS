/// LeetCode #266 - Palindrome Permutation
fn can_permute_palindrome(s: String) -> bool {
    let mut c = [0u8; 128];
    for b in s.bytes() {
        c[b as usize] ^= 1;
    }
    c.iter().filter(|&&x| x == 1).count() <= 1
}

fn main() {
    println!("{}", can_permute_palindrome("carerac".into()));
}

#[cfg(test)]
mod tests {
    use super::can_permute_palindrome;

    #[test]
    fn example_one() {
        assert!(can_permute_palindrome("carerac".into()));
    }

    #[test]
    fn example_two() {
        assert!(!can_permute_palindrome("code".into()));
    }
}
