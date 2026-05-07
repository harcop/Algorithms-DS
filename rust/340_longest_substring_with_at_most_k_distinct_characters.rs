/// LeetCode #340 - Longest Substring with At Most K Distinct Characters
use std::collections::HashMap;

fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
    let k = k as usize;
    let b = s.as_bytes();
    let mut cnt: HashMap<u8, usize> = HashMap::new();
    let mut lo = 0usize;
    let mut best = 0usize;
    for hi in 0..b.len() {
        *cnt.entry(b[hi]).or_insert(0) += 1;
        while cnt.len() > k {
            let e = cnt.get_mut(&b[lo]).unwrap();
            *e -= 1;
            if *e == 0 {
                cnt.remove(&b[lo]);
            }
            lo += 1;
        }
        best = best.max(hi - lo + 1);
    }
    best as i32
}

fn main() {
    println!("{}", length_of_longest_substring_k_distinct("eceba".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::length_of_longest_substring_k_distinct;

    #[test]
    fn examples() {
        assert_eq!(length_of_longest_substring_k_distinct("eceba".into(), 2), 3);
        assert_eq!(length_of_longest_substring_k_distinct("aa".into(), 1), 2);
    }
}
