/// LeetCode #1433 - Check If A String Can Break Another String
fn check_if_can_break(s1: String, s2: String) -> bool {
    fn sorted(s: &str) -> Vec<u8> {
        let mut v = s.as_bytes().to_vec();
        v.sort_unstable();
        v
    }
    fn ge(a: &[u8], b: &[u8]) -> bool {
        a.iter().zip(b.iter()).all(|(x, y)| x >= y)
    }
    let a = sorted(&s1);
    let b = sorted(&s2);
    ge(&a, &b) || ge(&b, &a)
}

fn main() {
    println!("{}", check_if_can_break("abc".into(), "xya".into()));
}

#[cfg(test)]
mod tests {
    use super::check_if_can_break;

    #[test]
    fn example_one() {
        assert!(check_if_can_break("abc".into(), "xya".into()));
    }

    #[test]
    fn example_two() {
        assert!(check_if_can_break("leetcode".into(), "code".into()));
    }
}

