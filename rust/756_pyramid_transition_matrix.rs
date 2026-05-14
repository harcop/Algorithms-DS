/// LeetCode #756 - Pyramid Transition Matrix
use std::collections::HashSet;

fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
    let mut rules: HashSet<(u8, u8, u8)> = HashSet::new();
    for w in allowed {
        let b = w.into_bytes();
        if b.len() == 3 {
            rules.insert((b[0], b[1], b[2]));
        }
    }
    let cur = bottom.into_bytes();

    fn dfs(cur: &[u8], rules: &HashSet<(u8, u8, u8)>) -> bool {
        if cur.len() == 1 {
            return true;
        }
        let mut next = Vec::with_capacity(cur.len() - 1);
        fn bt(cur: &[u8], pos: usize, next: &mut Vec<u8>, rules: &HashSet<(u8, u8, u8)>) -> bool {
            if pos == cur.len() - 1 {
                return dfs(next, rules);
            }
            let a = cur[pos];
            let b = cur[pos + 1];
            for c in b'A'..=b'Z' {
                if rules.contains(&(a, b, c)) {
                    next.push(c);
                    if bt(cur, pos + 1, next, rules) {
                        return true;
                    }
                    next.pop();
                }
            }
            false
        }
        bt(cur, 0, &mut next, rules)
    }

    dfs(&cur, &rules)
}

fn main() {
    println!(
        "{}",
        pyramid_transition(
            "BCD".into(),
            vec!["BCG".into(), "CDE".into(), "GEA".into(), "FFF".into()],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::pyramid_transition;

    #[test]
    fn example_one() {
        assert!(pyramid_transition(
            "BCD".into(),
            vec![
                "BCG".into(),
                "CDE".into(),
                "GEA".into(),
                "FFF".into(),
            ],
        ));
    }

    #[test]
    fn example_two() {
        assert!(!pyramid_transition(
            "AABA".into(),
            vec![
                "AAA".into(),
                "AAB".into(),
                "ABA".into(),
                "ABB".into(),
                "BAC".into(),
            ],
        ));
    }
}
