/// LeetCode #2278 - Percentage of Letter in String
fn percentage_letter(s: String, letter: char) -> i32 {
    let count = s.chars().filter(|&c| c == letter).count();
    100 * count as i32 / s.len() as i32
}

fn main() {
    println!("{}", percentage_letter("foobar".to_string(), 'o'));
}

#[cfg(test)]
mod tests {
    use super::percentage_letter;

    #[test]
    fn example_one() {
        assert_eq!(percentage_letter("foobar".to_string(), 'o'), 33);
    }

    #[test]
    fn example_two() {
        assert_eq!(percentage_letter("jjjj".to_string(), 'k'), 0);
    }
}
