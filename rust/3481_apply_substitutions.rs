/// LeetCode #3481 - Apply Substitutions
use std::collections::HashMap;

fn apply_substitutions(replacements: Vec<Vec<String>>, text: String) -> String {
    let d: HashMap<String, String> = replacements
        .into_iter()
        .map(|p| (p[0].clone(), p[1].clone()))
        .collect();
    fn dfs(s: &str, d: &HashMap<String, String>) -> String {
        let b = s.as_bytes();
        let mut i = 0;
        let mut out = String::new();
        while i < b.len() {
            if b[i] == b'%' {
                if let Some(rel) = s[i + 1..].find('%') {
                    let j = i + 1 + rel;
                    let key = &s[i + 1..j];
                    out.push_str(&dfs(&d[key], d));
                    i = j + 1;
                    continue;
                }
            }
            out.push(b[i] as char);
            i += 1;
        }
        out
    }
    dfs(&text, &d)
}

fn main() {
    println!(
        "{}",
        apply_substitutions(
            vec![vec!["A".into(), "abc".into()], vec!["B".into(), "def".into()]],
            "%A%_%B%".into()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::apply_substitutions;

    #[test]
    fn example1() {
        assert_eq!(
            apply_substitutions(
                vec![vec!["A".into(), "abc".into()], vec!["B".into(), "def".into()]],
                "%A%_%B%".into()
            ),
            "abc_def"
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            apply_substitutions(
                vec![
                    vec!["A".into(), "bce".into()],
                    vec!["B".into(), "ace".into()],
                    vec!["C".into(), "abc%B%".into()]
                ],
                "%A%_%B%_%C%".into()
            ),
            "bce_ace_abcace"
        );
    }
}
