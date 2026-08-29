/// LeetCode #3461 - Check If Digits Are Equal in String After Operations I
fn has_same_digits(s: String) -> bool {
    let mut t: Vec<u8> = s.bytes().map(|c| c - b'0').collect();
    let n = t.len();
    for k in (2..n).rev() {
        for i in 0..k {
            t[i] = (t[i] + t[i + 1]) % 10;
        }
    }
    t[0] == t[1]
}

fn main() {
    println!("{}", has_same_digits("3902".into()));
}

#[cfg(test)]
mod tests {
    use super::has_same_digits;

    #[test]
    fn example1() {
        assert!(has_same_digits("3902".into()));
    }

    #[test]
    fn example2() {
        assert!(!has_same_digits("34789".into()));
    }
}
