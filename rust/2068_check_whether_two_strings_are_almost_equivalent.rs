/// LeetCode #2068 - Check Whether Two Strings are Almost Equivalent
use std::collections::HashMap;

fn check_almost_equivalent(word1: String, word2: String) -> bool {
    let mut cnt: HashMap<u8, i32> = HashMap::new();
    for c in word1.bytes() {
        *cnt.entry(c).or_insert(0) += 1;
    }
    for c in word2.bytes() {
        *cnt.entry(c).or_insert(0) -= 1;
    }
    cnt.values().all(|&x| x.abs() <= 3)
}

fn main() {
    println!(
        "{}",
        check_almost_equivalent("aaaa".into(), "bccb".into())
    );
}

#[cfg(test)]
mod tests {
    use super::check_almost_equivalent;

    #[test]
    fn example_one() {
        assert!(!check_almost_equivalent("aaaa".into(), "bccb".into()));
    }

    #[test]
    fn example_two() {
        assert!(check_almost_equivalent("abcdeef".into(), "abaaacc".into()));
    }

    #[test]
    fn example_three() {
        assert!(check_almost_equivalent("cccddabba".into(), "babababab".into()));
    }
}
