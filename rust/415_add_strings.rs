/// LeetCode #415 - Add Strings
fn add_strings(num1: String, num2: String) -> String {
    let a = num1.as_bytes();
    let b = num2.as_bytes();
    let mut i = a.len();
    let mut j = b.len();
    let mut carry = 0u8;
    let mut out: Vec<u8> = vec![];
    while i > 0 || j > 0 || carry > 0 {
        let da = if i > 0 {
            i -= 1;
            a[i] - b'0'
        } else {
            0
        };
        let db = if j > 0 {
            j -= 1;
            b[j] - b'0'
        } else {
            0
        };
        let s = da + db + carry;
        out.push(b'0' + (s % 10));
        carry = s / 10;
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}

fn main() {
    println!("{}", add_strings("11".into(), "123".into()));
}

#[cfg(test)]
mod tests {
    use super::add_strings;

    #[test]
    fn example_one() {
        assert_eq!(add_strings("11".into(), "123".into()), "134");
    }
}
