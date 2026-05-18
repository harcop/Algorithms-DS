/// LeetCode #1056 - Confusing Number
fn confusing_number(n: i32) -> bool {
    let rotate = |d: u8| -> Option<u8> {
        match d {
            b'0' => Some(b'0'),
            b'1' => Some(b'1'),
            b'6' => Some(b'9'),
            b'8' => Some(b'8'),
            b'9' => Some(b'6'),
            _ => None,
        }
    };
    let s = n.to_string();
    let mut rev = String::new();
    for &b in s.as_bytes().iter().rev() {
        let Some(r) = rotate(b) else {
            return false;
        };
        rev.push(r as char);
    }
    let rotated: i64 = rev.parse().unwrap_or(0);
    rotated != n as i64
}

fn main() {
    println!("{}", confusing_number(6));
}

#[cfg(test)]
mod tests {
    use super::confusing_number;

    #[test]
    fn example_one() {
        assert!(confusing_number(6));
    }

    #[test]
    fn example_two() {
        assert!(!confusing_number(916));
    }
}
