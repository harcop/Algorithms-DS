/// LeetCode #159 - Longest Substring with At Most Two Distinct Characters
use std::collections::HashMap;

fn length_of_longest_substring_two_distinct(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut freq: HashMap<u8, usize> = HashMap::new();
    let mut lo = 0usize;
    let mut best = 0usize;

    for hi in 0..bytes.len() {
        *freq.entry(bytes[hi]).or_insert(0) += 1;
        while freq.len() > 2 {
            let c = bytes[lo];
            let e = freq.get_mut(&c).unwrap();
            *e -= 1;
            if *e == 0 {
                freq.remove(&c);
            }
            lo += 1;
        }
        best = best.max(hi - lo + 1);
    }
    best as i32
}

fn main() {
    println!("{}", length_of_longest_substring_two_distinct("eceba".into()));
}

#[cfg(test)]
mod tests {
    use super::length_of_longest_substring_two_distinct;

    #[test]
    fn example_one() {
        assert_eq!(length_of_longest_substring_two_distinct("eceba".into()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(length_of_longest_substring_two_distinct("ccaabbb".into()), 5);
    }
}
