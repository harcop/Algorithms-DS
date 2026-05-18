/// LeetCode #1044 - Longest Duplicate Substring
fn longest_dup_substring(s: String) -> String {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut lo = 1usize;
    let mut hi = n;
    let mut best = String::new();
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if let Some(found) = search(bytes, mid) {
            best = String::from_utf8(found).unwrap();
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    best
}

fn search(bytes: &[u8], len: usize) -> Option<Vec<u8>> {
    use std::collections::HashSet;
    if len == 0 {
        return None;
    }
    let mut seen = HashSet::new();
    for start in 0..=bytes.len() - len {
        let sub = bytes[start..start + len].to_vec();
        if !seen.insert(sub.clone()) {
            return Some(sub);
        }
    }
    None
}

fn main() {
    println!("{}", longest_dup_substring("banana".into()));
}

#[cfg(test)]
mod tests {
    use super::longest_dup_substring;

    #[test]
    fn example_one() {
        assert_eq!(longest_dup_substring("banana".into()), "ana");
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_dup_substring("abcd".into()), "");
    }
}
