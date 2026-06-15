/// LeetCode #1881 - Maximum Value after Insertion
fn max_value(n: String, x: i32) -> String {
    let x = x.to_string();
    let bytes = n.as_bytes();
    let mut i = 0usize;
    if bytes[0] == b'-' {
        i += 1;
        while i < bytes.len() && (bytes[i] - b'0') <= x.as_bytes()[0] - b'0' {
            i += 1;
        }
    } else {
        while i < bytes.len() && (bytes[i] - b'0') >= x.as_bytes()[0] - b'0' {
            i += 1;
        }
    }
    format!("{}{}{}", &n[..i], x, &n[i..])
}

fn main() {
    println!("{}", max_value("99".into(), 9));
}

#[cfg(test)]
mod tests {
    use super::max_value;

    #[test]
    fn example_one() {
        assert_eq!(max_value("99".into(), 9), "999");
    }
}
