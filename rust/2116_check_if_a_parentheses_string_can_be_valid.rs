/// LeetCode #2116 - Check if a Parentheses String Can Be Valid
fn can_be_valid(s: String, locked: String) -> bool {
    if s.len() % 2 == 1 {
        return false;
    }

    let s = s.as_bytes();
    let locked = locked.as_bytes();
    let mut balance = 0i32;

    for i in 0..s.len() {
        if locked[i] == b'0' || s[i] == b'(' {
            balance += 1;
        } else {
            balance -= 1;
        }
        if balance < 0 {
            return false;
        }
    }

    balance = 0;
    for i in (0..s.len()).rev() {
        if locked[i] == b'0' || s[i] == b')' {
            balance += 1;
        } else {
            balance -= 1;
        }
        if balance < 0 {
            return false;
        }
    }

    true
}

fn main() {
    println!("{}", can_be_valid("))()))".into(), "010100".into()));
}

#[cfg(test)]
mod tests {
    use super::can_be_valid;

    #[test]
    fn example_one() {
        assert!(can_be_valid("))()))".into(), "010100".into()));
    }

    #[test]
    fn example_two() {
        assert!(can_be_valid("()()".into(), "0000".into()));
    }

    #[test]
    fn example_three() {
        assert!(!can_be_valid(")".into(), "0".into()));
    }

    #[test]
    fn locked_prefix_can_make_string_invalid() {
        assert!(!can_be_valid("))".into(), "11".into()));
    }
}
