/// LeetCode #482 - License Key Formatting
fn license_key_formatting(s: String, k: i32) -> String {
    let k = k as usize;
    let chars: Vec<char> = s
        .chars()
        .filter(|&c| c != '-')
        .map(|c| c.to_ascii_uppercase())
        .collect();
    if chars.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    let first = chars.len() % k;
    let mut i = 0usize;
    if first != 0 {
        out.extend(chars[i..i + first].iter());
        i += first;
        if i < chars.len() {
            out.push('-');
        }
    }
    while i < chars.len() {
        out.extend(chars[i..i + k].iter());
        i += k;
        if i < chars.len() {
            out.push('-');
        }
    }
    out
}

fn main() {
    println!("{}", license_key_formatting("5F3Z-2e-9-w".into(), 4));
}

#[cfg(test)]
mod tests {
    use super::license_key_formatting;

    #[test]
    fn example_one() {
        assert_eq!(license_key_formatting("5F3Z-2e-9-w".into(), 4), "5F3Z-2E9W");
    }

    #[test]
    fn example_two() {
        assert_eq!(license_key_formatting("2-5g-3-J".into(), 2), "2-5G-3J");
    }
}
