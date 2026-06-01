/// LeetCode #1714 - Maximum Score From Removing Substrings
fn maximum_gain(s: String) -> i32 {
    let mut a = 0i64;
    let mut b = 0i64;
    let mut ans = 0i64;
    for c in s.bytes() {
        if c == b'a' {
            if b > 0 { b -= 1; ans += 1; } else { a += 1; }
        } else {
            if a > 0 { a -= 1; ans += 1; } else { b += 1; }
        }
    }
    ans as i32
}
fn main() { println!("{}", maximum_gain("cdbcbbaaabab".into())); }
#[cfg(test)]
mod tests {
    use super::maximum_gain;
    #[test]
    fn example_one() { assert_eq!(maximum_gain("cdbcbbaaabab".into()), 4); }
}