/// LeetCode #609 - Find Duplicate File in System
use std::collections::HashMap;

fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    for p in paths {
        let mut it = p.split_whitespace();
        let dir = it.next().unwrap();
        for rest in it {
            let (name, content) = rest.split_once('(').unwrap();
            let content = content.trim_end_matches(')');
            let full = format!("{}/{}", dir, name);
            m.entry(content.to_string()).or_default().push(full);
        }
    }
    m.into_values().filter(|v| v.len() > 1).collect()
}

fn main() {
    println!(
        "{:?}",
        find_duplicate(vec![
            "root/a 1.txt(abcd) 2.txt(efgh)".into(),
            "root/c 3.txt(abcd) 4.txt(efgh)".into(),
            "root/c/d 4.txt(efgh)".into(),
            "root 4.txt(efgh)".into(),
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::find_duplicate;

    #[test]
    fn example_one() {
        let mut v = find_duplicate(vec![
            "root/a 1.txt(abcd) 2.txt(efgh)".into(),
            "root/c 3.txt(abcd) 4.txt(efgh)".into(),
            "root/c/d 4.txt(efgh)".into(),
            "root 4.txt(efgh)".into(),
        ]);
        v.sort();
        for x in &mut v {
            x.sort();
        }
        let mut e: Vec<Vec<String>> = vec![
            vec![
                "root/a/1.txt".into(),
                "root/c/3.txt".into(),
            ],
            vec![
                "root/a/2.txt".into(),
                "root/c/d/4.txt".into(),
                "root/4.txt".into(),
                "root/c/4.txt".into(),
            ],
        ];
        e.sort();
        for x in &mut e {
            x.sort();
        }
        assert_eq!(v, e);
    }
}
