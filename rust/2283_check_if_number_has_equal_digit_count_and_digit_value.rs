/// LeetCode #2283 - Check if Number Has Equal Digit Count and Digit Value
fn digit_count(num: String) -> bool {
    let bytes = num.as_bytes();
    let mut freq = [0u8; 10];
    for &b in bytes {
        freq[(b - b'0') as usize] += 1;
    }
    for (i, &b) in bytes.iter().enumerate() {
        let expected = b - b'0';
        if freq[i] != expected {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", digit_count("1210".to_string()));
}

#[cfg(test)]
mod tests {
    use super::digit_count;

    #[test]
    fn example_one() {
        assert!(digit_count("1210".to_string()));
    }

    #[test]
    fn example_two() {
        assert!(!digit_count("030".to_string()));
    }
}

