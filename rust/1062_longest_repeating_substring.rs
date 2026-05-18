/// LeetCode #1062 - Longest Repeating Substring
fn longest_repeating_substring(s: String) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut lo = 0usize;
    let mut hi = n;
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        if has_dup(bytes, mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo as i32
}

fn has_dup(bytes: &[u8], len: usize) -> bool {
    use std::collections::HashSet;
    if len == 0 {
        return false;
    }
    let mut seen = HashSet::new();
    for i in 0..=bytes.len() - len {
        let sub = &bytes[i..i + len];
        if !seen.insert(sub) {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", longest_repeating_substring("abcd".into()));
}

#[cfg(test)]
mod tests {
    use super::longest_repeating_substring;

    #[test]
    fn example_one() {
        assert_eq!(longest_repeating_substring("abcd".into()), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_repeating_substring("abbaba".into()), 2);
    }
}
