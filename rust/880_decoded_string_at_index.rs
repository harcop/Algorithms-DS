/// LeetCode #880 - Decoded String at Index
fn decode_at_index(s: String, k: i32) -> String {
    let mut size = 0i64;
    for c in s.chars() {
        size = if c.is_ascii_digit() {
            size * (c as u8 - b'0') as i64
        } else {
            size + 1
        };
    }
    let mut k = k as i64;
    for c in s.chars().rev() {
        k %= size;
        if c.is_ascii_digit() {
            size /= (c as u8 - b'0') as i64;
        } else {
            if k == 0 || k == size {
                return c.to_string();
            }
            size -= 1;
        }
    }
    panic!("unreachable");
}

fn main() {
    println!("{}", decode_at_index("leet2code3".to_string(), 10));
}

#[cfg(test)]
mod tests {
    use super::decode_at_index;

    #[test]
    fn example_one() {
        assert_eq!(decode_at_index("leet2code3".to_string(), 10), "o");
    }
}
