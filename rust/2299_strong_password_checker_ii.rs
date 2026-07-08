/// LeetCode #2299 - Strong Password Checker II
fn strong_password_checker_ii(password: String) -> bool {
    let s = password.as_bytes();
    if s.len() < 8 {
        return false;
    }
    let mut mask = 0i32;
    for i in 0..s.len() {
        if i > 0 && s[i] == s[i - 1] {
            return false;
        }
        let c = s[i];
        if c.is_ascii_lowercase() {
            mask |= 1;
        } else if c.is_ascii_uppercase() {
            mask |= 2;
        } else if c.is_ascii_digit() {
            mask |= 4;
        } else {
            mask |= 8;
        }
    }
    mask == 15
}

fn main() {
    println!("{}", strong_password_checker_ii("IloveLe3tcode!".to_string()));
}

#[cfg(test)]
mod tests {
    use super::strong_password_checker_ii;

    #[test]
    fn example_one() {
        assert!(strong_password_checker_ii("IloveLe3tcode!".to_string()));
    }

    #[test]
    fn example_two() {
        assert!(!strong_password_checker_ii("Me+You--IsMyDream".to_string()));
    }

    #[test]
    fn example_three() {
        assert!(!strong_password_checker_ii("1aB!".to_string()));
    }
}
