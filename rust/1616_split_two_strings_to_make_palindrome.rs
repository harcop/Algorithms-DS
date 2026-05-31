/// LeetCode #1616 - Split Two Strings To Make Palindrome
fn check_palindrome_formation(a: String, b: String) -> bool {
    fn is_pal(s: &[u8]) -> bool { s.iter().eq(s.iter().rev()) }
    fn ok(a: &[u8], b: &[u8]) -> bool {
        let n = a.len();
        for i in 0..=n {
            for j in i..=n {
                if is_pal(&a[..i]) && is_pal(&b[i..j]) && is_pal(&a[j..]) { return true; }
                if is_pal(&b[..i]) && is_pal(&a[i..j]) && is_pal(&b[j..]) { return true; }
            }
        }
        false
    }
    let a = a.into_bytes();
    let b = b.into_bytes();
    ok(&a, &b) || ok(&b, &a)
}
fn main() { println!("{}", check_palindrome_formation("abc".into(), "def".into())); }
#[cfg(test)]
mod tests {
    use super::check_palindrome_formation;
    #[test]
    fn example_one() { assert!(check_palindrome_formation("abc".into(), "def".into())); }
}