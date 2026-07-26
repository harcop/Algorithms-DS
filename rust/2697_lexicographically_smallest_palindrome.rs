/// LeetCode #2697 - Lexicographically Smallest Palindrome
fn make_smallest_palindrome(s: String) -> String {
    let mut cs: Vec<u8> = s.into_bytes();
    let n = cs.len();
    let mut i = 0;
    let mut j = n - 1;
    while i < j {
        let m = cs[i].min(cs[j]);
        cs[i] = m;
        cs[j] = m;
        i += 1;
        j -= 1;
    }
    String::from_utf8(cs).unwrap()
}

fn main() {
    println!("{}", make_smallest_palindrome("egcfe".into()));
}

#[cfg(test)]
mod tests {
    use super::make_smallest_palindrome;

    #[test]
    fn example_one() {
        assert_eq!(make_smallest_palindrome("egcfe".into()), "efcfe");
    }

    #[test]
    fn example_two() {
        assert_eq!(make_smallest_palindrome("abcd".into()), "abba");
    }

    #[test]
    fn example_three() {
        assert_eq!(make_smallest_palindrome("seven".into()), "neven");
    }
}
