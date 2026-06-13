/// LeetCode #1839 - Longest Substring Of All Vowels in Order
fn longest_beautiful_substring(word: String) -> i32 {
    let word = word.as_bytes();
    let n = word.len();
    let mut ans = 0usize;
    let mut left = 0usize;
    let mut cnt = [0usize; 5];

    for right in 0..n {
        if right > 0 && word[right] < word[right - 1] {
            left = right;
            cnt = [0; 5];
        }
        cnt[vowel_idx(word[right])] += 1;
        if cnt.iter().all(|&c| c > 0) {
            ans = ans.max(right - left + 1);
        }
    }
    ans as i32
}

fn vowel_idx(c: u8) -> usize {
    match c {
        b'a' => 0,
        b'e' => 1,
        b'i' => 2,
        b'o' => 3,
        _ => 4,
    }
}

fn main() {
    println!(
        "{}",
        longest_beautiful_substring("aeiaaioaaaaeiiiiouuuooaauuaeiu".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::longest_beautiful_substring;

    #[test]
    fn example_one() {
        assert_eq!(
            longest_beautiful_substring("aeiaaioaaaaeiiiiouuuooaauuaeiu".to_string()),
            13
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            longest_beautiful_substring("aeeeiiiioooauuuaeiou".to_string()),
            5
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(longest_beautiful_substring("a".to_string()), 0);
    }
}
