/// LeetCode #1309 - Decrypt String from Alphabet to Integer Mapping
fn freq_alphabet(s: String) -> String {
    let mut i = 0;
    let b = s.as_bytes();
    let mut out = String::new();
    while i < b.len() {
        if i + 2 < b.len() && b[i + 2] == b'#' {
            let d = (b[i] - b'0') as i32 * 10 + (b[i + 1] - b'0') as i32;
            out.push((b'a' + (d - 1) as u8) as char);
            i += 3;
        } else {
            let d = (b[i] - b'0') as i32;
            out.push((b'a' + (d - 1) as u8) as char);
            i += 1;
        }
    }
    out
}

fn main() {
    println!("{}", freq_alphabet("10#11#12".to_string()));
}

#[cfg(test)]
mod tests {
    use super::freq_alphabet;

    #[test]
    fn example_one() {
        assert_eq!(freq_alphabet("10#11#12".to_string()), "jkab");
    }

    #[test]
    fn example_two() {
        assert_eq!(freq_alphabet("1326#".to_string()), "acz");
    }
}
