/// LeetCode #3137 - Minimum Number of Operations to Make Word K-Periodic
use std::collections::HashMap;

fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
    let k = k as usize;
    let n = word.len();
    let bytes = word.as_bytes();
    let mut cnt: HashMap<&[u8], i32> = HashMap::new();
    for i in (0..n).step_by(k) {
        *cnt.entry(&bytes[i..i + k]).or_insert(0) += 1;
    }
    let mx = *cnt.values().max().unwrap();
    (n / k) as i32 - mx
}

fn main() {
    println!(
        "{}",
        minimum_operations_to_make_k_periodic("leetcodeleet".into(), 4)
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_operations_to_make_k_periodic;

    #[test]
    fn example1() {
        assert_eq!(
            minimum_operations_to_make_k_periodic("leetcodeleet".into(), 4),
            1
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            minimum_operations_to_make_k_periodic("leetcoleet".into(), 2),
            3
        );
    }
}
