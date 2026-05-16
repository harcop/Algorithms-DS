/// LeetCode #856 - Score of Parentheses
fn score_of_parentheses(s: String) -> i32 {
    let mut stack = vec![0];
    for c in s.chars() {
        if c == '(' {
            stack.push(0);
        } else {
            let v = stack.pop().unwrap();
            let top = stack.last_mut().unwrap();
            *top += if v == 0 { 1 } else { 2 * v };
        }
    }
    stack[0]
}

fn main() {
    println!("{}", score_of_parentheses("(()".into()));
}

#[cfg(test)]
mod tests {
    use super::score_of_parentheses;

    #[test]
    fn example_one() {
        assert_eq!(score_of_parentheses("(()".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(score_of_parentheses("(())".into()), 2);
    }
}
