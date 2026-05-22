/// LeetCode #1233 - Remove Sub-Folders from the Filesystem
fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
    let mut f = folder;
    f.sort_unstable();
    let mut ans = Vec::new();
    for path in f {
        if let Some(last) = ans.last() {
            let prefix = format!("{}/", last);
            if path.starts_with(&prefix) {
                continue;
            }
        }
        ans.push(path);
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        remove_subfolders(vec![
            "/a".into(),
            "/a/b".into(),
            "/c/d".into(),
            "/c/d/e".into(),
            "/c/f".into(),
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::remove_subfolders;

    #[test]
    fn example_one() {
        assert_eq!(
            remove_subfolders(vec![
                "/a".into(),
                "/a/b".into(),
                "/c/d".into(),
                "/c/d/e".into(),
                "/c/f".into(),
            ]),
            vec!["/a".to_string(), "/c/d".to_string(), "/c/f".to_string()]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            remove_subfolders(vec!["/a".into(), "/a/b/c".into(), "/a/b/d".into()]),
            vec!["/a".to_string()]
        );
    }
}
