/// LeetCode #3227 - Vowels Game in a String
fn does_alice_win(s: String) -> bool {
    s.chars().any(|c| "aeiou".contains(c))
}

fn main() {
    println!("{}", does_alice_win("leetcoder".into()));
}

#[cfg(test)]
mod tests {
    use super::does_alice_win;

    #[test]
    fn example1() {
        assert!(does_alice_win("leetcoder".into()));
    }

    #[test]
    fn example2() {
        assert!(!does_alice_win("bbcd".into()));
    }
}
