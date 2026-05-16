/// LeetCode #831 - Masking Personal Information
fn mask_pii(s: String) -> String {
    if s.contains('@') {
        let lower = s.to_lowercase();
        let at = lower.find('@').unwrap();
        let local = &lower[..at];
        let domain = &lower[at + 1..];
        format!("{}*****@{}", &local[..1], domain)
    } else {
        let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.len() == 10 {
            format!("***-***-{}", &digits[6..])
        } else {
            format!(
                "+{}-***-***-{}",
                digits.len() - 10,
                &digits[digits.len() - 4..]
            )
        }
    }
}

fn main() {
    println!("{}", mask_pii("1abc@def.com".into()));
}

#[cfg(test)]
mod tests {
    use super::mask_pii;

    #[test]
    fn example_email() {
        assert_eq!(mask_pii("1abc@def.com".into()), "1*****@def.com");
    }

    #[test]
    fn example_phone() {
        assert_eq!(mask_pii("123-456-7890".into()), "***-***-7890");
    }
}
