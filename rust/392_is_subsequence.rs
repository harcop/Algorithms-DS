/// LeetCode #392 - Is Subsequence
fn is_subsequence(s: String, t: String) -> bool {
    let sb = s.as_bytes();
    let tb = t.as_bytes();
    let mut i = 0usize;
    for &b in tb {
        if i < sb.len() && sb[i] == b {
            i += 1;
        }
    }
    i == sb.len()
}

fn main() {
    println!("{}", is_subsequence("abc".into(), "ahbgdc".into()));
}

#[cfg(test)]
mod tests {
    use super::is_subsequence;

    #[test]
    fn example_one() {
        assert!(is_subsequence("abc".into(), "ahbgdc".into()));
    }

    #[test]
    fn example_two() {
        assert!(!is_subsequence("axc".into(), "ahbgdc".into()));
    }
}
