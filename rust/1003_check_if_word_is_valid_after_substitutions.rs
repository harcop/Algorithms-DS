/// LeetCode #1003 - Check If Word Is Valid After Substitutions
fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.bytes() {
        stack.push(c);
        if stack.len() >= 3
            && stack[stack.len() - 3] == b'a'
            && stack[stack.len() - 2] == b'b'
            && stack[stack.len() - 1] == b'c'
        {
            stack.truncate(stack.len() - 3);
        }
    }
    stack.is_empty()
}

fn main() {
    println!("{}", is_valid("aabcbc".into()));
}

#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test]
    fn example_one() {
        assert!(is_valid("aabcbc".into()));
    }

    #[test]
    fn example_two() {
        assert!(!is_valid("abcabcababcc".into()));
    }
}
