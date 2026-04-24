/// LeetCode #32 - Longest Valid Parentheses
fn longest_valid_parentheses(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut stack: Vec<i32> = vec![-1];
    let mut best = 0i32;

    for (i, &ch) in bytes.iter().enumerate() {
        if ch == b'(' {
            stack.push(i as i32);
        } else {
            stack.pop();
            if let Some(&last) = stack.last() {
                best = best.max(i as i32 - last);
            } else {
                stack.push(i as i32);
            }
        }
    }

    best
}

fn main() {
    println!("{}", longest_valid_parentheses(")()())".to_string()));
}

#[cfg(test)]
mod tests {
    use super::longest_valid_parentheses;

    #[test]
    fn example_one() {
        assert_eq!(longest_valid_parentheses("(()".to_string()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_valid_parentheses(")()())".to_string()), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(longest_valid_parentheses("".to_string()), 0);
    }
}
