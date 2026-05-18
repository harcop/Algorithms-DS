/// LeetCode #1021 - Remove Outermost Parentheses
fn remove_outer_parentheses(s: String) -> String {
    let mut depth = 0i32;
    let mut out = String::new();
    for c in s.chars() {
        if c == '(' {
            if depth > 0 {
                out.push(c);
            }
            depth += 1;
        } else {
            depth -= 1;
            if depth > 0 {
                out.push(c);
            }
        }
    }
    out
}

fn main() {
    println!("{}", remove_outer_parentheses("(()())(())".into()));
}

#[cfg(test)]
mod tests {
    use super::remove_outer_parentheses;

    #[test]
    fn example_one() {
        assert_eq!(remove_outer_parentheses("(()())(())".into()), "()()()");
    }

    #[test]
    fn example_two() {
        assert_eq!(remove_outer_parentheses("(()())(())(()(()))".into()), "()()()()(())");
    }
}
