/// LeetCode #1784 - Check If Binary String Has at Most One Segment of Ones
fn check_ones_segment(s: String) -> bool {
    let b = s.as_bytes();
    let mut seen_one = false;
    let mut seen_zero_after_one = false;
    for &ch in b {
        if ch == b'1' {
            if seen_zero_after_one {
                return false;
            }
            seen_one = true;
        } else if seen_one {
            seen_zero_after_one = true;
        }
    }
    true
}
fn main() { println!("{}", check_ones_segment("1001".into())); }
#[cfg(test)]
mod tests {
    use super::check_ones_segment;
    #[test]
    fn example_one() {
        assert!(!check_ones_segment("1001".into()));
    }
    #[test]
    fn example_two() {
        assert!(check_ones_segment("100".into()));
    }
}
