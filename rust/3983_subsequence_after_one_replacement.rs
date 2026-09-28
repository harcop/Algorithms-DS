/// LeetCode #3983 - Subsequence After One Replacement
fn can_make_subsequence(s: String, t: String) -> bool {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let m = s.len();
    let mut i0 = 0usize;
    let mut i1 = 0usize;
    let mut j = 0usize;
    while i1 < m && j < t.len() {
        if s[i1] == t[j] {
            i1 += 1;
        }
        if i1 < i0 + 1 {
            i1 = i0 + 1;
        }
        if i0 < m && s[i0] == t[j] {
            i0 += 1;
        }
        j += 1;
    }
    i1 == m
}

fn main() {
    println!(
        "{}",
        can_make_subsequence("cat".into(), "chat".into())
    );
}

#[cfg(test)]
mod tests {
    use super::can_make_subsequence;

    #[test]
    fn example1() {
        assert!(can_make_subsequence("cat".into(), "chat".into()));
    }

    #[test]
    fn example2() {
        assert!(!can_make_subsequence("plane".into(), "apple".into()));
    }

    #[test]
    fn already_subsequence() {
        assert!(can_make_subsequence("abc".into(), "aabbcc".into()));
    }

    #[test]
    fn too_long() {
        assert!(!can_make_subsequence("abcd".into(), "abc".into()));
    }
}
