/// LeetCode #796 - Rotate String
fn rotate_string(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }
    let doubled = format!("{}{}", s, s);
    doubled.contains(&goal)
}

fn main() {
    println!("{}", rotate_string("abcde".into(), "cdeab".into()));
}

#[cfg(test)]
mod tests {
    use super::rotate_string;

    #[test]
    fn example_one() {
        assert!(rotate_string("abcde".into(), "cdeab".into()));
    }

    #[test]
    fn example_two() {
        assert!(!rotate_string("abcde".into(), "abced".into()));
    }
}
