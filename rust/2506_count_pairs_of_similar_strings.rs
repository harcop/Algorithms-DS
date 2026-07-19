/// LeetCode #2506 - Count Pairs Of Similar Strings
use std::collections::HashMap;

fn similar_pairs(words: Vec<String>) -> i32 {
    let mut ans = 0;
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for s in words {
        let mut x = 0;
        for c in s.bytes() {
            x |= 1 << (c - b'a');
        }
        ans += *cnt.get(&x).unwrap_or(&0);
        *cnt.entry(x).or_insert(0) += 1;
    }
    ans
}

fn main() {
    println!(
        "{}",
        similar_pairs(vec![
            "aba".to_string(),
            "aabb".to_string(),
            "abcd".to_string(),
            "bac".to_string(),
            "aabc".to_string()
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::similar_pairs;

    #[test]
    fn example_one() {
        assert_eq!(
            similar_pairs(vec![
                "aba".to_string(),
                "aabb".to_string(),
                "abcd".to_string(),
                "bac".to_string(),
                "aabc".to_string()
            ]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            similar_pairs(vec![
                "aabb".to_string(),
                "ab".to_string(),
                "ba".to_string()
            ]),
            3
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            similar_pairs(vec![
                "nba".to_string(),
                "cba".to_string(),
                "dba".to_string()
            ]),
            0
        );
    }
}
