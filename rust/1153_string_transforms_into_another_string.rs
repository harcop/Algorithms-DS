/// LeetCode #1153 - String Transforms Into Another String
use std::collections::{HashMap, HashSet};

fn can_convert(str1: String, str2: String) -> bool {
    if str1 == str2 {
        return true;
    }
    let a: Vec<char> = str1.chars().collect();
    let b: Vec<char> = str2.chars().collect();
    let mut map = HashMap::new();
    for i in 0..a.len() {
        if let Some(&prev) = map.get(&a[i]) {
            if prev != b[i] {
                return false;
            }
        } else {
            map.insert(a[i], b[i]);
        }
    }
    b.iter().copied().collect::<HashSet<_>>().len() < 26
}

fn main() {
    println!("{}", can_convert("aabcc".into(), "ccdee".into()));
}

#[cfg(test)]
mod tests {
    use super::can_convert;

    #[test]
    fn example_one() {
        assert!(can_convert("aabcc".into(), "ccdee".into()));
    }

    #[test]
    fn example_two() {
        assert!(!can_convert("leetcode".into(), "codeleet".into()));
    }
}
