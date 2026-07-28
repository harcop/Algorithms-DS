/// LeetCode #2744 - Find Maximum Number of String Pairs
use std::collections::HashMap;

fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut ans = 0;
    for w in words {
        let a = (w.as_bytes()[0] - b'a') as i32;
        let b = (w.as_bytes()[1] - b'a') as i32;
        ans += cnt.get(&(b << 5 | a)).copied().unwrap_or(0);
        *cnt.entry(a << 5 | b).or_insert(0) += 1;
    }
    ans
}

fn main() {
    println!(
        "{}",
        maximum_number_of_string_pairs(vec![
            "cd".into(),
            "ac".into(),
            "dc".into(),
            "ca".into(),
            "zz".into()
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_number_of_string_pairs;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_number_of_string_pairs(vec![
                "cd".into(),
                "ac".into(),
                "dc".into(),
                "ca".into(),
                "zz".into()
            ]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximum_number_of_string_pairs(vec!["ab".into(), "ba".into(), "cc".into()]),
            1
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            maximum_number_of_string_pairs(vec!["aa".into(), "ab".into()]),
            0
        );
    }
}
