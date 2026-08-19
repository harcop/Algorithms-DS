/// LeetCode #3306 - Count of Substrings Containing Every Vowel and K Consonants II
use std::collections::HashMap;

fn count_of_substrings(word: String, k: i32) -> i64 {
    fn f(word: &[u8], k: i32) -> i64 {
        let mut ans = 0i64;
        let mut l = 0;
        let mut x = 0;
        let mut cnt = HashMap::new();
        let is_vowel = |c: u8| matches!(c, b'a' | b'e' | b'i' | b'o' | b'u');
        for (r, &c) in word.iter().enumerate() {
            if is_vowel(c) {
                *cnt.entry(c).or_insert(0) += 1;
            } else {
                x += 1;
            }
            while x >= k && cnt.len() == 5 {
                let d = word[l];
                l += 1;
                if is_vowel(d) {
                    let e = cnt.entry(d).or_insert(0);
                    *e -= 1;
                    if *e == 0 {
                        cnt.remove(&d);
                    }
                } else {
                    x -= 1;
                }
            }
            ans += l as i64;
        }
        ans
    }
    let b = word.as_bytes();
    f(b, k) - f(b, k + 1)
}

fn main() {
    println!("{}", count_of_substrings("ieaouqqieaouqq".into(), 1));
}

#[cfg(test)]
mod tests {
    use super::count_of_substrings;

    #[test]
    fn example1() {
        assert_eq!(count_of_substrings("aeioqq".into(), 1), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(count_of_substrings("aeiou".into(), 0), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(count_of_substrings("ieaouqqieaouqq".into(), 1), 3);
    }
}
