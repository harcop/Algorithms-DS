/// LeetCode #937 - Reorder Data in Log Files

fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
    let mut logs = logs;
    logs.sort_by(|a, b| {
        let (ida, ra) = {
            let sp = a.find(' ').unwrap();
            (&a[..sp], &a[sp + 1..])
        };
        let (idb, rb) = {
            let sp = b.find(' ').unwrap();
            (&b[..sp], &b[sp + 1..])
        };
        let la = ra.chars().next().unwrap().is_alphabetic();
        let lb = rb.chars().next().unwrap().is_alphabetic();
        match (la, lb) {
            (false, true) => std::cmp::Ordering::Greater,
            (true, false) => std::cmp::Ordering::Less,
            (false, false) => std::cmp::Ordering::Equal,
            (true, true) => ra.cmp(rb).then(ida.cmp(idb)),
        }
    });
    logs
}

fn main() {
    println!("{:?}", reorder_log_files(vec![
        "a1 9 2 3 1".into(),
        "g1 act car".into(),
        "zo4 4 7".into(),
        "ab1 off key dog".into(),
        "a8 act zoo".into(),
    ]));
}

#[cfg(test)]
mod tests {
    use super::reorder_log_files;

    #[test]
    fn example_one() {
        let out = reorder_log_files(vec![
            "a1 9 2 3 1".into(),
            "g1 act car".into(),
            "zo4 4 7".into(),
            "ab1 off key dog".into(),
            "a8 act zoo".into(),
        ]);
        assert_eq!(
            out,
            vec![
                "g1 act car".to_string(),
                "a8 act zoo".to_string(),
                "ab1 off key dog".to_string(),
                "a1 9 2 3 1".to_string(),
                "zo4 4 7".to_string(),
            ]
        );
    }
}
