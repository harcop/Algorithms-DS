/// LeetCode #2794 - Create Object from Two Arrays (JS problem; Rust analogue)
use std::collections::HashMap;

fn create_object(keys: Vec<String>, values: Vec<i32>) -> HashMap<String, i32> {
    let mut ans = HashMap::new();
    for (k, v) in keys.into_iter().zip(values) {
        ans.entry(k).or_insert(v);
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        create_object(vec!["a".into(), "b".into(), "c".into()], vec![1, 2, 3])
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn example_one() {
        assert_eq!(
            create_object(vec!["a".into(), "b".into(), "c".into()], vec![1, 2, 3]),
            HashMap::from([
                ("a".to_string(), 1),
                ("b".to_string(), 2),
                ("c".to_string(), 3),
            ])
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            create_object(vec!["1".into(), "1".into(), "false".into()], vec![4, 5, 6]),
            HashMap::from([("1".to_string(), 4), ("false".to_string(), 6)])
        );
    }

    #[test]
    fn example_three() {
        assert!(create_object(vec![], vec![]).is_empty());
    }
}
