/// LeetCode #3329 - Count Substrings With K-Frequency Characters II
fn number_of_substrings(s: String, k: i32) -> i64 {
    let s = s.as_bytes();
    let mut cnt = [0i32; 26];
    let mut ans = 0i64;
    let mut l = 0usize;
    for &c in s {
        let i = (c - b'a') as usize;
        cnt[i] += 1;
        while cnt[i] >= k {
            cnt[(s[l] - b'a') as usize] -= 1;
            l += 1;
        }
        ans += l as i64;
    }
    ans
}

fn main() {
    println!("{}", number_of_substrings("abacb".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::number_of_substrings;

    #[test]
    fn example1() {
        assert_eq!(number_of_substrings("abacb".into(), 2), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(number_of_substrings("abcde".into(), 1), 15);
    }
}
