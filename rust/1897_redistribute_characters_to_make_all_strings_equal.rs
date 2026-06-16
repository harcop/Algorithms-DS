/// LeetCode #1897 - Redistribute Characters to Make All Strings Equal
fn make_equal(words: Vec<String>) -> bool {
    let mut cnt = [0i32; 26];
    for w in &words {
        for c in w.bytes() {
            cnt[(c - b'a') as usize] += 1;
        }
    }
    let n = words.len() as i32;
    cnt.iter().all(|&v| v % n == 0)
}

fn main() {
    println!(
        "{}",
        make_equal(vec!["abc".into(), "aabc".into(), "bc".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::make_equal;

    #[test]
    fn example_one() {
        assert!(make_equal(vec!["abc".into(), "aabc".into(), "bc".into()]));
    }

    #[test]
    fn example_two() {
        assert!(!make_equal(vec!["ab".into(), "a".into()]));
    }
}
