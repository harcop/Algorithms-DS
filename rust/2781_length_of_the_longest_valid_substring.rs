/// LeetCode #2781 - Length of the Longest Valid Substring
use std::collections::HashSet;

fn longest_valid_substring(word: &str, forbidden: Vec<String>) -> i32 {
    let s: HashSet<&str> = forbidden.iter().map(|x| x.as_str()).collect();
    let bytes = word.as_bytes();
    let n = bytes.len();
    let mut ans = 0;
    let mut i: usize = 0;
    for j in 0..n {
        let start = if j > 10 { j - 10 } else { 0 };
        let lower = if i > 0 { start.max(i - 1) } else { start };
        for k in (lower..=j).rev() {
            if s.contains(&word[k..=j]) {
                i = k + 1;
                break;
            }
        }
        if i <= j {
            ans = ans.max((j - i + 1) as i32);
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        longest_valid_substring("cbaaaabc", vec!["aaa".into(), "cb".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::longest_valid_substring;

    #[test]
    fn example_one() {
        assert_eq!(
            longest_valid_substring("cbaaaabc", vec!["aaa".into(), "cb".into()]),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            longest_valid_substring(
                "leetcode",
                vec!["de".into(), "le".into(), "e".into()]
            ),
            4
        );
    }
}
