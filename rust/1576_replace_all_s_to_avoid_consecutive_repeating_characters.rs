/// LeetCode #1576 - Replace All S To Avoid Consecutive Repeating Characters
fn modify_string(s: String) -> String {
    let mut a: Vec<u8> = s.into_bytes();
    for i in 0..a.len() {
        if a[i] == b'?' {
            for c in b'a'..=b'z' {
                let ok = (i == 0 || a[i - 1] != c)
                    && (i + 1 >= a.len() || a[i + 1] != c)
                    && (i < 2 || a[i - 1] != c || a[i - 2] != c)
                    && (i + 2 >= a.len() || a[i + 1] != c || a[i + 2] != c);
                if ok { a[i] = c; break; }
            }
        }
    }
    String::from_utf8(a).unwrap()
}
fn main() { println!("{}", modify_string("?zs".into())); }
#[cfg(test)]
mod tests {
    use super::modify_string;
    #[test]
    fn example_one() { assert_eq!(modify_string("?zs".into()), "azs"); }
    #[test]
    fn example_two() { assert_eq!(modify_string("ubv?w".into()), "ubvaw"); }
}