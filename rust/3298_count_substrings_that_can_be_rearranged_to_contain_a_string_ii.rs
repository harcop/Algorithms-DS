/// LeetCode #3298 - Count Substrings That Can Be Rearranged to Contain a String II
fn valid_substring_count(word1: String, word2: String) -> i64 {
    if word1.len() < word2.len() {
        return 0;
    }
    let mut cnt = [0i32; 26];
    let mut need = 0i32;
    for b in word2.bytes() {
        let i = (b - b'a') as usize;
        if cnt[i] == 0 {
            need += 1;
        }
        cnt[i] += 1;
    }
    let mut win = [0i32; 26];
    let mut ans = 0i64;
    let mut l = 0usize;
    let w = word1.as_bytes();
    for &c in w {
        let i = (c - b'a') as usize;
        win[i] += 1;
        if win[i] == cnt[i] {
            need -= 1;
        }
        while need == 0 {
            let j = (w[l] - b'a') as usize;
            if win[j] == cnt[j] {
                need += 1;
            }
            win[j] -= 1;
            l += 1;
        }
        ans += l as i64;
    }
    ans
}

fn main() {
    println!("{}", valid_substring_count("abcabc".into(), "abc".into()));
}

#[cfg(test)]
mod tests {
    use super::valid_substring_count;

    #[test]
    fn example1() {
        assert_eq!(valid_substring_count("bcca".into(), "abc".into()), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(valid_substring_count("abcabc".into(), "abc".into()), 10);
    }

    #[test]
    fn example3() {
        assert_eq!(valid_substring_count("abcabc".into(), "aaabc".into()), 0);
    }
}
