/// LeetCode #17 - Letter Combinations of a Phone Number
fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let map = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    let mut result: Vec<String> = vec![String::new()];

    for d in digits.bytes() {
        let idx = (d - b'0') as usize;
        let letters = map[idx].as_bytes();
        let mut next = Vec::new();

        for prefix in &result {
            for &ch in letters {
                let mut s = prefix.clone();
                s.push(ch as char);
                next.push(s);
            }
        }
        result = next;
    }

    result
}

fn main() {
    println!("{:?}", letter_combinations("23".to_string()));
}

#[cfg(test)]
mod tests {
    use super::letter_combinations;

    #[test]
    fn example_one() {
        let mut got = letter_combinations("23".to_string());
        got.sort();
        let mut expected = vec![
            "ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf",
        ]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
        expected.sort();
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        assert!(letter_combinations("".to_string()).is_empty());
    }

    #[test]
    fn example_three() {
        assert_eq!(letter_combinations("2".to_string()), vec!["a", "b", "c"]);
    }
}
