/// LeetCode #1297 - Maximum Number of Occurrences of a Substring
use std::collections::HashMap;

fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let min_size = min_size as usize;
    let max_letters = max_letters as usize;
    let mut best = 0;
    for size in min_size..=(max_size.min(n as i32) as usize) {
        
        let mut freq = [0usize; 26];
        let mut distinct = 0usize;
        for i in 0..size {
            let idx = (s[i] - b'a') as usize;
            if freq[idx] == 0 {
                distinct += 1;
            }
            freq[idx] += 1;
        }
        let mut counts: HashMap<Vec<u8>, i32> = HashMap::new();
        if distinct <= max_letters {
            let key = s[0..size].to_vec();
            *counts.entry(key.clone()).or_insert(0) += 1;
            best = best.max(counts[&key]);
        }
        for i in size..n {
            let out = (s[i - size] - b'a') as usize;
            freq[out] -= 1;
            if freq[out] == 0 {
                distinct -= 1;
            }
            let inc = (s[i] - b'a') as usize;
            if freq[inc] == 0 {
                distinct += 1;
            }
            freq[inc] += 1;
            if distinct <= max_letters {
                let key = s[i + 1 - size..=i].to_vec();
                *counts.entry(key.clone()).or_insert(0) += 1;
                best = best.max(counts[&key]);
            }
        }
    }
    best
}

fn main() {
    println!("{}", max_freq("aababcaab".to_string(), 2, 3, 4));
}

#[cfg(test)]
mod tests {
    use super::max_freq;

    #[test]
    fn example_one() {
        assert_eq!(max_freq("aababcaab".to_string(), 2, 3, 4), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_freq("aaaa".to_string(), 1, 3, 3), 2);
    }
}
