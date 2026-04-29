/// LeetCode #67 - Add Binary
fn add_binary(a: String, b: String) -> String {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut i = a.len();
    let mut j = b.len();
    let mut carry = 0u8;
    let mut out = Vec::new();

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
        let sum = da + db + carry;
        out.push((sum % 2) + b'0');
        carry = sum / 2;
    }

    out.reverse();
    String::from_utf8(out).unwrap()
}

fn main() {
    println!("{}", add_binary("11".to_string(), "1".to_string()));
}

#[cfg(test)]
mod tests {
    use super::add_binary;

    #[test]
    fn example_one() {
        assert_eq!(add_binary("11".to_string(), "1".to_string()), "100");
    }

    #[test]
    fn example_two() {
        assert_eq!(add_binary("1010".to_string(), "1011".to_string()), "10101");
    }
}
