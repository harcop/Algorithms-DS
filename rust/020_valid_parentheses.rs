/// LeetCode #20 - Valid Parentheses
fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();

    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => return false,
        }
    }

    stack.is_empty()
}

fn main() {
    println!("{}", is_valid("()[]{}".to_string()));
}

#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test]
    fn example_one() {
        assert!(is_valid("()".to_string()));
    }

    #[test]
    fn example_two() {
        assert!(is_valid("()[]{}".to_string()));
    }

    #[test]
    fn example_three() {
        assert!(!is_valid("(]".to_string()));
    }
}
