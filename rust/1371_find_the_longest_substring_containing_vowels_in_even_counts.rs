/// LeetCode #1371 - Find The Longest Substring Containing Vowels In Even Counts

use std::collections::HashMap;

fn vowel_idx(c: u8) -> usize {
    match c {
        b'a' => 0,
        b'e' => 1,
        b'i' => 2,
        b'o' => 3,
        _ => 4,
    }
}

fn is_vowel(c: u8) -> bool {
    matches!(c, b'a' | b'e' | b'i' | b'o' | b'u')
}

fn find_the_longest_substring(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut seen = HashMap::new();
    seen.insert(0usize, -1i32);
    let mut mask = 0usize;
    let mut ans = 0i32;
    for (i, &c) in bytes.iter().enumerate() {
        if is_vowel(c) {
            mask ^= 1 << vowel_idx(c);
        }
        let key = mask;
        if let Some(&j) = seen.get(&key) {
            ans = ans.max(i as i32 - j);
        } else {
            seen.insert(key, i as i32);
        }
    }
    ans
}

fn main() {
    println!("{}", find_the_longest_substring("eleetminicarorio".into()));
}

#[cfg(test)]
mod tests {
    use super::find_the_longest_substring;

    #[test]
    fn example_one() {
        assert_eq!(find_the_longest_substring("eleetminicarorio".into()), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_the_longest_substring("bcbcbc".into()), 6);
    }
}
