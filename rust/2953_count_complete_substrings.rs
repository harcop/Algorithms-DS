/// LeetCode #2953 - Count Complete Substrings
fn count_complete_substrings(word: String, k: i32) -> i32 {
    fn f(s: &[u8], k: usize) -> i32 {
        let m = s.len();
        let mut ans = 0;
        for i in 1..=26 {
            let l = i * k;
            if l > m {
                break;
            }
            let mut cnt = [0i32; 26];
            let mut freq = [0i32; 100_001];
            for &c in &s[..l] {
                let idx = (c - b'a') as usize;
                cnt[idx] += 1;
            }
            for &c in &cnt {
                if c > 0 {
                    freq[c as usize] += 1;
                }
            }
            if freq[k] == i as i32 {
                ans += 1;
            }
            for j in l..m {
                let add = (s[j] - b'a') as usize;
                freq[cnt[add] as usize] -= 1;
                cnt[add] += 1;
                freq[cnt[add] as usize] += 1;

                let rem = (s[j - l] - b'a') as usize;
                freq[cnt[rem] as usize] -= 1;
                cnt[rem] -= 1;
                if cnt[rem] > 0 {
                    freq[cnt[rem] as usize] += 1;
                }

                if freq[k] == i as i32 {
                    ans += 1;
                }
            }
        }
        ans
    }

    let bytes = word.as_bytes();
    let n = bytes.len();
    let mut ans = 0;
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && (bytes[j] as i32 - bytes[j - 1] as i32).abs() <= 2 {
            j += 1;
        }
        ans += f(&bytes[i..j], k as usize);
        i = j;
    }
    ans
}

fn main() {
    println!("{}", count_complete_substrings("igigee".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::count_complete_substrings;

    #[test]
    fn example_one() {
        assert_eq!(count_complete_substrings("igigee".into(), 2), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_complete_substrings("aaabbbccc".into(), 3), 6);
    }
}
