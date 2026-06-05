/// LeetCode #1736 - Latest Time by Replacing Hidden Digits
fn maximum_time(time: String) -> String {
    let mut t = time.into_bytes();
    if t[0] == b'?' {
        t[0] = if t[1] >= b'4' && t[1] <= b'9' { b'1' } else { b'2' };
    }
    if t[1] == b'?' {
        t[1] = if t[0] == b'2' { b'3' } else { b'9' };
    }
    if t[3] == b'?' {
        t[3] = b'5';
    }
    if t[4] == b'?' {
        t[4] = b'9';
    }
    String::from_utf8(t).unwrap()
}
fn main() { println!("{}", maximum_time("2?:?0".into())); }
#[cfg(test)]
mod tests {
    use super::maximum_time;
    #[test]
    fn example_one() { assert_eq!(maximum_time("2?:?0".into()), "23:50"); }
    #[test]
    fn example_two() { assert_eq!(maximum_time("0?:3?".into()), "09:39"); }
    #[test]
    fn example_three() { assert_eq!(maximum_time("1?:22".into()), "19:22"); }
}
