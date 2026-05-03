/// LeetCode #249 - Group Shifted Strings
use std::collections::HashMap;

fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<Vec<i32>, Vec<String>> = HashMap::new();
    for s in strings {
        let b = s.as_bytes();
        let mut key = Vec::with_capacity(b.len());
        let base = b[0] as i32;
        for &c in b {
            key.push((c as i32 - base + 26) % 26);
        }
        map.entry(key).or_default().push(s);
    }
    let mut groups: Vec<Vec<String>> = map.into_values().collect();
    for g in &mut groups {
        g.sort();
    }
    groups.sort_by(|a, b| a[0].cmp(&b[0]));
    groups
}

fn main() {
    println!("{:?}", group_strings(vec!["abc".into(), "bcd".into(), "acef".into(), "xyz".into()]));
}

#[cfg(test)]
mod tests {
    use super::group_strings;

    #[test]
    fn example_one() {
        let mut g = group_strings(vec![
            "abc".into(),
            "bcd".into(),
            "acef".into(),
            "xyz".into(),
            "az".into(),
            "ba".into(),
            "a".into(),
            "z".into(),
        ]);
        g.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut e1: Vec<String> = vec!["a".into(), "z".into()];
        e1.sort();
        let mut e2: Vec<String> = vec!["abc".into(), "bcd".into(), "xyz".into()];
        e2.sort();
        let e3: Vec<String> = vec!["acef".into()];
        let mut e4: Vec<String> = vec!["az".into(), "ba".into()];
        e4.sort();
        let mut exp: Vec<Vec<String>> = vec![e1, e2, e3, e4];
        exp.sort_by(|a, b| a[0].cmp(&b[0]));
        assert_eq!(g, exp);
    }
}
