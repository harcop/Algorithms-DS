/// LeetCode #2631 - Group By (JS problem; Rust HashMap analogue)
use std::collections::HashMap;

fn group_by<T, F>(arr: Vec<T>, fn_: F) -> HashMap<String, Vec<T>>
where
    F: Fn(&T) -> String,
{
    let mut acc: HashMap<String, Vec<T>> = HashMap::new();
    for item in arr {
        let key = fn_(&item);
        acc.entry(key).or_default().push(item);
    }
    acc
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5, 6];
    let g = group_by(arr, |n| (n > &5).to_string());
    println!("{:?}", g);
}

#[cfg(test)]
mod tests {
    use super::group_by;
    use std::collections::HashMap;

    #[test]
    fn example_ids() {
        let arr = vec![
            HashMap::from([("id".into(), "1".into())]),
            HashMap::from([("id".into(), "1".into())]),
            HashMap::from([("id".into(), "2".into())]),
        ];
        let g = group_by(arr, |item: &HashMap<String, String>| item["id"].clone());
        assert_eq!(g["1"].len(), 2);
        assert_eq!(g["2"].len(), 1);
    }

    #[test]
    fn example_threshold() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let g = group_by(arr, |n| (*n > 5).to_string());
        assert_eq!(g["true"], vec![6, 7, 8, 9, 10]);
        assert_eq!(g["false"], vec![1, 2, 3, 4, 5]);
    }
}
