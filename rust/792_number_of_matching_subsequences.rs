/// LeetCode #792 - Number of Matching Subsequences
use std::collections::HashMap;

fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
    let b = s.as_bytes();
    let mut waiting: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    let mut count = 0i32;
    for (i, w) in words.iter().enumerate() {
        let wb = w.as_bytes();
        if wb.is_empty() {
            count += 1;
            continue;
        }
        waiting.entry((wb[0] - b'a') as usize).or_default().push((i, 0));
    }
    for &c in b {
        let idx = (c - b'a') as usize;
        let list = waiting.remove(&idx).unwrap_or_default();
        let mut nxt: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        for (wi, pos) in list {
            let wb = words[wi].as_bytes();
            let np = pos + 1;
            if np == wb.len() {
                count += 1;
            } else {
                nxt.entry((wb[np] - b'a') as usize).or_default().push((wi, np));
            }
        }
        for (k, v) in nxt {
            waiting.entry(k).or_default().extend(v);
        }
    }
    count
}

fn main() {
    println!(
        "{}",
        num_matching_subseq("abcde".into(), vec!["a".into(), "bb".into(), "acd".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::num_matching_subseq;

    #[test]
    fn example_one() {
        assert_eq!(
            num_matching_subseq("abcde".into(), vec!["a".into(), "bb".into(), "acd".into()]),
            2
        );
    }
}
