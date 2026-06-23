/// LeetCode #2053 - Kth Distinct String in an Array
use std::collections::HashMap;

fn kth_distinct(arr: Vec<String>, k: i32) -> String {
    let mut cnt = HashMap::new();
    for s in &arr {
        *cnt.entry(s.clone()).or_insert(0) += 1;
    }
    let mut k = k;
    for s in arr {
        if cnt[&s] == 1 {
            k -= 1;
            if k == 0 {
                return s;
            }
        }
    }
    String::new()
}

fn main() {
    println!(
        "{}",
        kth_distinct(vec!["d".into(), "b".into(), "c".into(), "b".into(), "c".into(), "a".into()], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::kth_distinct;

    #[test]
    fn example_one() {
        assert_eq!(
            kth_distinct(
                vec!["d".into(), "b".into(), "c".into(), "b".into(), "c".into(), "a".into()],
                2,
            ),
            "a"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            kth_distinct(vec!["aaa".into(), "aa".into(), "a".into()], 1),
            "aaa"
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(kth_distinct(vec!["a".into(), "b".into(), "a".into()], 3), "");
    }
}
