/// LeetCode #2724 - Sort By (JS problem; Rust sort-by-key analogue)
fn sort_by<T, F>(mut arr: Vec<T>, fn_key: F) -> Vec<T>
where
    F: Fn(&T) -> i64,
{
    arr.sort_by_key(|x| fn_key(x));
    arr
}

fn main() {
    println!("{:?}", sort_by(vec![5, 4, 1, 2, 3], |x| *x as i64));
}

#[cfg(test)]
mod tests {
    use super::sort_by;
    use std::collections::BTreeMap;

    #[test]
    fn example_one() {
        assert_eq!(sort_by(vec![5, 4, 1, 2, 3], |x| *x as i64), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn example_two() {
        let arr = vec![
            BTreeMap::from([("x".to_string(), 1i64)]),
            BTreeMap::from([("x".to_string(), 0i64)]),
            BTreeMap::from([("x".to_string(), -1i64)]),
        ];
        let sorted = sort_by(arr, |d: &BTreeMap<String, i64>| *d.get("x").unwrap_or(&0));
        assert_eq!(
            sorted,
            vec![
                BTreeMap::from([("x".to_string(), -1i64)]),
                BTreeMap::from([("x".to_string(), 0i64)]),
                BTreeMap::from([("x".to_string(), 1i64)]),
            ]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            sort_by(vec![vec![3, 4], vec![5, 2], vec![10, 1]], |x| x[1] as i64),
            vec![vec![10, 1], vec![5, 2], vec![3, 4]]
        );
    }
}
