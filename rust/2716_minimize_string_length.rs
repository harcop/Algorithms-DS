/// LeetCode #2716 - Minimize String Length
use std::collections::HashSet;

fn minimized_string_length(s: String) -> i32 {
    s.chars().collect::<HashSet<_>>().len() as i32
}

fn main() {
    println!("{}", minimized_string_length("aaabc".into()));
}

#[cfg(test)]
mod tests {
    use super::minimized_string_length;

    #[test]
    fn example_one() {
        assert_eq!(minimized_string_length("aaabc".into()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimized_string_length("cbbd".into()), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimized_string_length("baadccab".into()), 4);
    }
}
