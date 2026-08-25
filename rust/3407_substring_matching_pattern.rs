/// LeetCode #3407 - Substring Matching Pattern
fn has_match(s: String, p: String) -> bool {
    let mut i = 0;
    for t in p.split('*') {
        if let Some(j) = s[i..].find(t) {
            i += j + t.len();
        } else {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", has_match("leetcode".into(), "ee*e".into()));
}

#[cfg(test)]
mod tests {
    use super::has_match;

    #[test]
    fn example1() {
        assert!(has_match("leetcode".into(), "ee*e".into()));
    }

    #[test]
    fn example2() {
        assert!(!has_match("car".into(), "c*v".into()));
    }

    #[test]
    fn example3() {
        assert!(has_match("luck".into(), "u*".into()));
    }
}
