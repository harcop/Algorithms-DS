/// LeetCode #2168 - Unique Substrings With Equal Digit Frequency
use std::collections::HashSet;

fn equal_digit_frequency(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut seen = HashSet::new();

    for i in 0..bytes.len() {
        let mut freq = [0i32; 10];
        let mut unique = 0i32;
        let mut max_freq = 0i32;

        for j in i..bytes.len() {
            let d = (bytes[j] - b'0') as usize;
            if freq[d] == 0 {
                unique += 1;
            }
            freq[d] += 1;
            max_freq = max_freq.max(freq[d]);

            let len = (j - i + 1) as i32;
            if max_freq * unique == len {
                seen.insert(&s[i..=j]);
            }
        }
    }

    seen.len() as i32
}

fn main() {
    println!("{}", equal_digit_frequency("1212".into()));
}

#[cfg(test)]
mod tests {
    use super::equal_digit_frequency;

    #[test]
    fn example_one() {
        assert_eq!(equal_digit_frequency("1212".into()), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(equal_digit_frequency("12321".into()), 9);
    }
}
