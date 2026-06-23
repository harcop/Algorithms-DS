/// LeetCode #2062 - Count Vowel Substrings of a String
fn count_vowel_substrings(word: String) -> i32 {
    let vowels: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
    let word = word.as_bytes();
    let n = word.len();
    let mut ans = 0i32;

    for i in 0..n {
        let mut seen = [false; 5];
        let mut distinct = 0usize;
        for &c in &word[i..] {
            let mut is_vowel = false;
            for (j, &v) in vowels.iter().enumerate() {
                if c == v {
                    if !seen[j] {
                        seen[j] = true;
                        distinct += 1;
                    }
                    is_vowel = true;
                    break;
                }
            }
            if !is_vowel {
                break;
            }
            if distinct == 5 {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", count_vowel_substrings("aeiouu".into()));
}

#[cfg(test)]
mod tests {
    use super::count_vowel_substrings;

    #[test]
    fn example_one() {
        assert_eq!(count_vowel_substrings("aeiouu".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_vowel_substrings("unicornarihan".into()), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_vowel_substrings("cuaieuouac".into()), 7);
    }
}
