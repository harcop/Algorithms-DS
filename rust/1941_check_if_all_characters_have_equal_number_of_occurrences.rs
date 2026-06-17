/// LeetCode #1941 - Check if All Characters Have Equal Number of Occurrences
fn are_occurrences_equal(s: String) -> bool {
    let mut cnt = [0i32; 26];
    for b in s.bytes() {
        cnt[(b - b'a') as usize] += 1;
    }
    let mut seen: Option<i32> = None;
    for &c in &cnt {
        if c == 0 {
            continue;
        }
        if let Some(v) = seen {
            if v != c {
                return false;
            }
        } else {
            seen = Some(c);
        }
    }
    true
}

fn main() {
    println!("{}", are_occurrences_equal("abacbc".into()));
}

#[cfg(test)]
mod tests {
    use super::are_occurrences_equal;

    #[test]
    fn example_one() {
        assert!(are_occurrences_equal("abacbc".into()));
    }

    #[test]
    fn example_two() {
        assert!(!are_occurrences_equal("aaabb".into()));
    }
}
