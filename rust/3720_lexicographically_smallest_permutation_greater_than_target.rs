/// LeetCode #3720 - Lexicographically Smallest Permutation Greater Than Target
fn lex_greater_permutation(s: String, target: String) -> String {
    fn nxt(cnt: &[i32], x: u8) -> Option<u8> {
        let start = (x - b'a' + 1) as usize;
        for i in start..26 {
            if cnt[i] > 0 {
                return Some(b'a' + i as u8);
            }
        }
        None
    }

    let mut cnt = [0i32; 26];
    for c in s.bytes() {
        cnt[(c - b'a') as usize] += 1;
    }
    let mut tmp = cnt;
    let tbytes = target.as_bytes();
    let mut j = None;
    for (i, &x) in tbytes.iter().enumerate() {
        if nxt(&tmp, x).is_some() {
            j = Some(i);
        }
        let idx = (x - b'a') as usize;
        if tmp[idx] == 0 {
            break;
        }
        tmp[idx] -= 1;
    }
    let Some(j) = j else {
        return String::new();
    };
    let mut result = Vec::new();
    for i in 0..j {
        result.push(tbytes[i]);
        cnt[(tbytes[i] - b'a') as usize] -= 1;
    }
    let y = nxt(&cnt, tbytes[j]).unwrap();
    result.push(y);
    cnt[(y - b'a') as usize] -= 1;
    for i in 0..26 {
        for _ in 0..cnt[i] {
            result.push(b'a' + i as u8);
        }
    }
    String::from_utf8(result).unwrap()
}

fn main() {
    println!("{}", lex_greater_permutation("abc".into(), "bba".into()));
}

#[cfg(test)]
mod tests {
    use super::lex_greater_permutation;

    #[test]
    fn example1() {
        assert_eq!(
            lex_greater_permutation("abc".into(), "bba".into()),
            "bca"
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            lex_greater_permutation("leet".into(), "code".into()),
            "eelt"
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            lex_greater_permutation("baba".into(), "bbaa".into()),
            ""
        );
    }
}
