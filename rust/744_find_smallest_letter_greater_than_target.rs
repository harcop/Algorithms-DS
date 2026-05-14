/// LeetCode #744 - Find Smallest Letter Greater Than Target
fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    for &c in &letters {
        if c > target {
            return c;
        }
    }
    letters[0]
}

fn main() {
    println!(
        "{}",
        next_greatest_letter(vec!['c', 'f', 'j'], 'a')
    );
}

#[cfg(test)]
mod tests {
    use super::next_greatest_letter;

    #[test]
    fn example_one() {
        assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'a'), 'c');
    }

    #[test]
    fn example_two() {
        assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'c'), 'f');
    }

    #[test]
    fn example_three() {
        assert_eq!(next_greatest_letter(vec!['x', 'x', 'y', 'y'], 'z'), 'x');
    }
}
