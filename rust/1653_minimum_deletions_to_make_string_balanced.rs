/// LeetCode #1653 - Minimum Deletions To Make String Balanced
fn minimum_deletions(s: String) -> i32 {
    let mut b = 0i32;
    let mut ans = 0i32;
    for c in s.bytes() {
        if c == b'b' { b += 1; }
        else if b > 0 { ans += 1; b -= 1; }
    }
    ans
}
fn main() { println!("{}", minimum_deletions("aababbab".into())); }
#[cfg(test)]
mod tests {
    use super::minimum_deletions;
    #[test]
    fn example_one() { assert_eq!(minimum_deletions("aababbab".into()), 2); }
}