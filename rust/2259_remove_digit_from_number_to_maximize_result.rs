/// LeetCode #2259 - Remove Digit From Number to Maximize Result
fn remove_digit(number: String, digit: char) -> String {
    let chars: Vec<char> = number.chars().collect();
    for i in 0..chars.len().saturating_sub(1) {
        if chars[i] == digit && digit < chars[i + 1] {
            return chars
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, &c)| c)
                .collect();
        }
    }

    if let Some(pos) = chars.iter().rposition(|&c| c == digit) {
        return chars
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != pos)
            .map(|(_, &c)| c)
            .collect();
    }

    number
}

fn main() {
    println!("{}", remove_digit("123".to_string(), '3'));
}

#[cfg(test)]
mod tests {
    use super::remove_digit;

    #[test]
    fn example_one() {
        assert_eq!(remove_digit("123".to_string(), '3'), "12");
    }

    #[test]
    fn example_two() {
        assert_eq!(remove_digit("1231".to_string(), '1'), "231");
    }

    #[test]
    fn example_three() {
        assert_eq!(remove_digit("551".to_string(), '5'), "51");
    }
}
