/// LeetCode #3794 - Reverse String Prefix
fn reverse_prefix(s: String, k: i32) -> String {
    let k = k as usize;
    let mut t: Vec<u8> = s.into_bytes();
    t[..k].reverse();
    String::from_utf8(t).unwrap()
}

fn main() {
    println!("{}", reverse_prefix("abcd".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::reverse_prefix;

    #[test]
    fn example1() {
        assert_eq!(reverse_prefix("abcd".into(), 2), "bacd");
    }

    #[test]
    fn example2() {
        assert_eq!(reverse_prefix("xyz".into(), 3), "zyx");
    }

    #[test]
    fn example3() {
        assert_eq!(reverse_prefix("hey".into(), 1), "hey");
    }
}
