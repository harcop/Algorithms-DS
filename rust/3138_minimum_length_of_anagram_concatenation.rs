/// LeetCode #3138 - Minimum Length of Anagram Concatenation
fn min_anagram_length(s: String) -> i32 {
    let n = s.len();
    let bytes = s.as_bytes();
    let mut total = [0i32; 26];
    for &c in bytes {
        total[(c - b'a') as usize] += 1;
    }
    let check = |k: usize| -> bool {
        let groups = n / k;
        for i in (0..n).step_by(k) {
            let mut cnt = [0i32; 26];
            for &c in &bytes[i..i + k] {
                cnt[(c - b'a') as usize] += 1;
            }
            for c in 0..26 {
                if cnt[c] * groups as i32 != total[c] {
                    return false;
                }
            }
        }
        true
    };
    for i in 1..=n {
        if n % i == 0 && check(i) {
            return i as i32;
        }
    }
    n as i32
}

fn main() {
    println!("{}", min_anagram_length("abba".into()));
}

#[cfg(test)]
mod tests {
    use super::min_anagram_length;

    #[test]
    fn example1() {
        assert_eq!(min_anagram_length("abba".into()), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_anagram_length("cdef".into()), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(min_anagram_length("abcbcacabbaccba".into()), 3);
    }
}
