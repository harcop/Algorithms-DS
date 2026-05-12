/// LeetCode #678 - Valid Parenthesis String
fn check_valid_string(s: String) -> bool {
    let mut lo = 0i32;
    let mut hi = 0i32;
    for c in s.chars() {
        match c {
            '(' => {
                lo += 1;
                hi += 1;
            }
            ')' => {
                lo = (lo - 1).max(0);
                hi -= 1;
            }
            _ => {
                lo = (lo - 1).max(0);
                hi += 1;
            }
        }
        if hi < 0 {
            return false;
        }
    }
    lo == 0
}

fn main() {
    println!("{}", check_valid_string("(*))".into()));
}

#[cfg(test)]
mod tests {
    use super::check_valid_string;

    #[test]
    fn example_one() {
        assert!(check_valid_string("()".into()));
    }

    #[test]
    fn example_two() {
        assert!(check_valid_string("(*)".into()));
    }

    #[test]
    fn example_three() {
        assert!(check_valid_string("(*))".into()));
    }

    #[test]
    fn example_four() {
        assert!(!check_valid_string(")(".into()));
    }
}
