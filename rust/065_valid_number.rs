/// LeetCode #65 - Valid Number
fn is_number(s: String) -> bool {
    let s = s.trim();
    if s.is_empty() {
        return false;
    }

    let bytes = s.as_bytes();
    let mut seen_digit = false;
    let mut seen_dot = false;
    let mut seen_exp = false;
    let mut digit_after_exp = true;

    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'0'..=b'9' => {
                seen_digit = true;
                if seen_exp {
                    digit_after_exp = true;
                }
            }
            b'+' | b'-' => {
                if i != 0 && bytes[i - 1] != b'e' && bytes[i - 1] != b'E' {
                    return false;
                }
            }
            b'.' => {
                if seen_dot || seen_exp {
                    return false;
                }
                seen_dot = true;
            }
            b'e' | b'E' => {
                if seen_exp || !seen_digit {
                    return false;
                }
                seen_exp = true;
                digit_after_exp = false;
            }
            _ => return false,
        }
    }

    seen_digit && digit_after_exp
}

fn main() {
    println!("{}", is_number("2e10".to_string()));
}

#[cfg(test)]
mod tests {
    use super::is_number;

    #[test]
    fn valid_cases() {
        for s in [
            "2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1",
            "53.5e93", "-123.456e789",
        ] {
            assert!(is_number(s.to_string()), "failed on {}", s);
        }
    }

    #[test]
    fn invalid_cases() {
        for s in ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"] {
            assert!(!is_number(s.to_string()), "failed on {}", s);
        }
    }
}
