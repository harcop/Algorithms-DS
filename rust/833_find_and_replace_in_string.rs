/// LeetCode #833 - Find And Replace in String
fn find_replace_string(
    s: String,
    indices: Vec<i32>,
    sources: Vec<String>,
    targets: Vec<String>,
) -> String {
    let mut order: Vec<usize> = (0..indices.len()).collect();
    order.sort_by_key(|&i| indices[i]);
    let mut res = String::new();
    let mut pos = 0usize;
    let b = s.as_bytes();
    for &i in &order {
        let idx = indices[i] as usize;
        res.push_str(&s[pos..idx]);
        if &b[idx..idx + sources[i].len()] == sources[i].as_bytes() {
            res.push_str(&targets[i]);
        } else {
            res.push_str(&s[idx..idx + sources[i].len()]);
        }
        pos = idx + sources[i].len();
    }
    res.push_str(&s[pos..]);
    res
}

fn main() {
    println!(
        "{}",
        find_replace_string(
            "abcd".into(),
            vec![0, 2],
            vec!["a".into(), "cd".into()],
            vec!["eee".into(), "ffff".into()],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::find_replace_string;

    #[test]
    fn example_one() {
        assert_eq!(
            find_replace_string(
                "abcd".into(),
                vec![0, 2],
                vec!["a".into(), "cd".into()],
                vec!["eee".into(), "ffff".into()],
            ),
            "eeebffff"
        );
    }
}
