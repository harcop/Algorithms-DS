/// LeetCode #2675 - Array of Objects to Matrix (JS problem; Rust enum analogue)
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq)]
enum Json {
    Null,
    Bool(bool),
    Number(i64),
    String(String),
    Array(Vec<Json>),
    Object(BTreeMap<String, Json>),
}

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Empty,
    Null,
    Bool(bool),
    Number(i64),
    String(String),
}

fn flatten(key: &str, obj: &Json, out: &mut BTreeMap<String, Cell>) {
    match obj {
        Json::Null => {
            out.insert(key.to_string(), Cell::Null);
        }
        Json::Bool(b) => {
            out.insert(key.to_string(), Cell::Bool(*b));
        }
        Json::Number(n) => {
            out.insert(key.to_string(), Cell::Number(*n));
        }
        Json::String(s) => {
            out.insert(key.to_string(), Cell::String(s.clone()));
        }
        Json::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                let new_key = if key.is_empty() {
                    i.to_string()
                } else {
                    format!("{key}.{i}")
                };
                flatten(&new_key, v, out);
            }
        }
        Json::Object(map) => {
            for (k, v) in map {
                let new_key = if key.is_empty() {
                    k.clone()
                } else {
                    format!("{key}.{k}")
                };
                flatten(&new_key, v, out);
            }
        }
    }
}

fn json_to_matrix(arr: &[Json]) -> Vec<Vec<Cell>> {
    let rows: Vec<BTreeMap<String, Cell>> = arr
        .iter()
        .map(|obj| {
            let mut m = BTreeMap::new();
            flatten("", obj, &mut m);
            m
        })
        .collect();
    let mut keys = BTreeSet::new();
    for row in &rows {
        keys.extend(row.keys().cloned());
    }
    let keys: Vec<String> = keys.into_iter().collect();
    let mut ans = vec![keys.iter().map(|k| Cell::String(k.clone())).collect()];
    for row in &rows {
        let mut new_row = Vec::with_capacity(keys.len());
        for k in &keys {
            new_row.push(row.get(k).cloned().unwrap_or(Cell::Empty));
        }
        ans.push(new_row);
    }
    // empty keys case: still one header + one row per object, all empty
    if keys.is_empty() {
        ans = vec![vec![]; arr.len() + 1];
    }
    ans
}

fn main() {
    let mut a = BTreeMap::new();
    a.insert("b".into(), Json::Number(1));
    a.insert("a".into(), Json::Number(2));
    let mut b = BTreeMap::new();
    b.insert("b".into(), Json::Number(3));
    b.insert("a".into(), Json::Number(4));
    println!("{:?}", json_to_matrix(&[Json::Object(a), Json::Object(b)]));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn example_one() {
        let mut a = BTreeMap::new();
        a.insert("b".into(), Json::Number(1));
        a.insert("a".into(), Json::Number(2));
        let mut b = BTreeMap::new();
        b.insert("b".into(), Json::Number(3));
        b.insert("a".into(), Json::Number(4));
        assert_eq!(
            json_to_matrix(&[Json::Object(a), Json::Object(b)]),
            vec![
                vec![Cell::String("a".into()), Cell::String("b".into())],
                vec![Cell::Number(2), Cell::Number(1)],
                vec![Cell::Number(4), Cell::Number(3)],
            ]
        );
    }

    #[test]
    fn example_empty() {
        assert_eq!(
            json_to_matrix(&[
                Json::Object(BTreeMap::new()),
                Json::Object(BTreeMap::new()),
                Json::Object(BTreeMap::new()),
            ]),
            vec![vec![], vec![], vec![], vec![]]
        );
    }
}
