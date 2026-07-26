/// LeetCode #2722 - Join Two Arrays by ID (JS problem; Rust map merge analogue)
use std::collections::BTreeMap;

fn join(
    arr1: Vec<BTreeMap<String, i64>>,
    arr2: Vec<BTreeMap<String, i64>>,
) -> Vec<BTreeMap<String, i64>> {
    let mut map: BTreeMap<i64, BTreeMap<String, i64>> = BTreeMap::new();
    for obj in arr1 {
        let id = *obj.get("id").unwrap_or(&0);
        map.insert(id, obj);
    }
    for obj in arr2 {
        let id = *obj.get("id").unwrap_or(&0);
        map.entry(id)
            .and_modify(|existing| {
                for (k, v) in &obj {
                    existing.insert(k.clone(), *v);
                }
            })
            .or_insert(obj);
    }
    map.into_values().collect()
}

fn obj(pairs: &[(&str, i64)]) -> BTreeMap<String, i64> {
    pairs.iter().map(|(k, v)| ((*k).into(), *v)).collect()
}

fn main() {
    println!(
        "{:?}",
        join(
            vec![obj(&[("id", 1), ("x", 1)]), obj(&[("id", 2), ("x", 9)])],
            vec![obj(&[("id", 3), ("x", 5)])]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::{join, obj};

    #[test]
    fn example_one() {
        assert_eq!(
            join(
                vec![obj(&[("id", 1), ("x", 1)]), obj(&[("id", 2), ("x", 9)])],
                vec![obj(&[("id", 3), ("x", 5)])]
            ),
            vec![
                obj(&[("id", 1), ("x", 1)]),
                obj(&[("id", 2), ("x", 9)]),
                obj(&[("id", 3), ("x", 5)]),
            ]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            join(
                vec![
                    obj(&[("id", 1), ("x", 2), ("y", 3)]),
                    obj(&[("id", 2), ("x", 3), ("y", 6)]),
                ],
                vec![
                    obj(&[("id", 2), ("x", 10), ("y", 20)]),
                    obj(&[("id", 3), ("x", 0), ("y", 0)]),
                ]
            ),
            vec![
                obj(&[("id", 1), ("x", 2), ("y", 3)]),
                obj(&[("id", 2), ("x", 10), ("y", 20)]),
                obj(&[("id", 3), ("x", 0), ("y", 0)]),
            ]
        );
    }
}
