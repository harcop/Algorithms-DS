/// LeetCode #3805 - Count Caesar Cipher Pairs
use std::collections::HashMap;

fn count_pairs(words: Vec<String>) -> i64 {
    let mut cnt: HashMap<Vec<u8>, i64> = HashMap::new();
    for s in words {
        let mut t = s.into_bytes();
        let k = (b'z' - t[0]) as u8;
        for i in 1..t.len() {
            t[i] = b'a' + (t[i] - b'a' + k) % 26;
        }
        t[0] = b'z';
        *cnt.entry(t).or_insert(0) += 1;
    }
    cnt.values().map(|&v| v * (v - 1) / 2).sum()
}

fn main() {
    println!(
        "{}",
        count_pairs(vec!["fusion".into(), "layout".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::count_pairs;

    #[test]
    fn example1() {
        assert_eq!(
            count_pairs(vec!["fusion".into(), "layout".into()]),
            1
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_pairs(vec!["ab".into(), "aa".into(), "za".into(), "aa".into()]),
            2
        );
    }
}
