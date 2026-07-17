/// LeetCode #2423 - Remove Letter To Equalize Frequency
fn equal_frequency(word: String) -> bool {
    let mut frequency = [0; 26];
    for byte in word.bytes() {
        frequency[(byte - b'a') as usize] += 1;
    }

    for i in 0..26 {
        if frequency[i] == 0 {
            continue;
        }
        frequency[i] -= 1;
        let target = frequency.iter().copied().find(|&count| count > 0);
        if frequency
            .iter()
            .all(|&count| count == 0 || Some(count) == target)
        {
            return true;
        }
        frequency[i] += 1;
    }

    false
}

fn main() {
    println!("{}", equal_frequency("abcc".to_string()));
}

#[cfg(test)]
mod tests {
    use super::equal_frequency;

    #[test]
    fn example_one() {
        assert!(equal_frequency("abcc".to_string()));
    }

    #[test]
    fn example_two() {
        assert!(!equal_frequency("aazz".to_string()));
    }
}
