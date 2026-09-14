/// LeetCode #3734 - Lexicographically Smallest Palindromic Permutation Greater Than Target
fn lex_palindromic_permutation(s: String, target: String) -> String {
    let mut freq = [0usize; 26];
    s.bytes().for_each(|ch| freq[(ch - b'a') as usize] += 1);
    if freq.iter().filter(|&&cnt| cnt & 1 != 0).count() > 1 {
        return String::new();
    }
    let mid = freq.iter().position(|cnt| cnt & 1 != 0);
    freq.iter_mut().for_each(|cnt| *cnt /= 2);
    let mut ans = s.into_bytes();
    let tgt = target.into_bytes();
    let half = ans.len() / 2;
    let make = |buf: &mut [u8]| {
        if let Some(ch) = mid {
            buf[half] = b'a' + ch as u8;
        }
        let len = buf.len();
        for idx in 0..half {
            buf[len - 1 - idx] = buf[idx];
        }
    };
    let mut pos = 0;
    while pos < half {
        let ch = (tgt[pos] - b'a') as usize;
        if freq[ch] == 0 {
            break;
        }
        ans[pos] = tgt[pos];
        freq[ch] -= 1;
        pos += 1;
    }
    if pos == half {
        make(&mut ans);
        if ans.as_slice() > tgt.as_slice() {
            return String::from_utf8(ans).unwrap();
        }
    }
    loop {
        if pos < half {
            let min = (tgt[pos] - b'a' + 1) as usize;
            if let Some(ch) = (min..26).find(|&ch| freq[ch] != 0) {
                ans[pos] = b'a' + ch as u8;
                freq[ch] -= 1;
                let mut dst = pos + 1;
                for (ch, &cnt) in freq.iter().enumerate() {
                    for off in 0..cnt {
                        ans[dst + off] = b'a' + ch as u8;
                    }
                    dst += cnt;
                }
                make(&mut ans);
                return String::from_utf8(ans).unwrap();
            }
        }
        if pos == 0 {
            return String::new();
        }
        pos -= 1;
        freq[(tgt[pos] - b'a') as usize] += 1;
    }
}

fn main() {
    println!("{}", lex_palindromic_permutation("baba".into(), "abba".into()));
}

#[cfg(test)]
mod tests {
    use super::lex_palindromic_permutation;

    #[test]
    fn example1() {
        assert_eq!(
            lex_palindromic_permutation("baba".into(), "abba".into()),
            "baab"
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            lex_palindromic_permutation("baba".into(), "bbaa".into()),
            ""
        );
    }

    #[test]
    fn example3() {
        assert_eq!(lex_palindromic_permutation("abc".into(), "abb".into()), "");
    }

    #[test]
    fn example4() {
        assert_eq!(
            lex_palindromic_permutation("aac".into(), "abb".into()),
            "aca"
        );
    }
}
