/// LeetCode #2822 - Inversion of Object (JS problem; Rust analogue)
use std::collections::HashMap;

fn invert_object(obj: HashMap<String, String>) -> HashMap<String, Vec<String>> {
    let mut inv: HashMap<String, Vec<String>> = HashMap::new();
    for (k, v) in obj {
        inv.entry(v).or_default().push(k);
    }
    for keys in inv.values_mut() {
        keys.sort();
    }
    inv
}

fn main() {
    let mut obj = HashMap::new();
    obj.insert("a".into(), "1".into());
    obj.insert("b".into(), "2".into());
    obj.insert("c".into(), "2".into());
    obj.insert("d".into(), "4".into());
    let inv = invert_object(obj);
    println!("{:?}", inv.get("2"));
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::invert_object;

    #[test]
    fn example_one() {
        let mut obj = HashMap::new();
        obj.insert("a".into(), "1".into());
        obj.insert("b".into(), "2".into());
        obj.insert("c".into(), "2".into());
        obj.insert("d".into(), "4".into());
        let inv = invert_object(obj);
        assert_eq!(inv.get("1").unwrap(), &vec!["a".to_string()]);
        assert_eq!(inv.get("2").unwrap(), &vec!["b".to_string(), "c".to_string()]);
        assert_eq!(inv.get("4").unwrap(), &vec!["d".to_string()]);
    }
}
