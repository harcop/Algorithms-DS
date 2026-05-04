/// LeetCode #291 - Word Pattern II
use std::collections::HashMap;

fn word_pattern_match(pattern: String, s: String) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let sb = s.as_bytes();
    let mut p2s: HashMap<char, String> = HashMap::new();
    let mut used = HashMap::<String, char>::new();

    fn dfs(
        pi: usize,
        si: usize,
        p: &[char],
        sb: &[u8],
        p2s: &mut HashMap<char, String>,
        used: &mut HashMap<String, char>,
    ) -> bool {
        if pi == p.len() && si == sb.len() {
            return true;
        }
        if pi == p.len() || si == sb.len() {
            return false;
        }
        let c = p[pi];
        if let Some(m) = p2s.get(&c) {
            let m = m.as_bytes();
            if si + m.len() <= sb.len() && &sb[si..si + m.len()] == m {
                return dfs(pi + 1, si + m.len(), p, sb, p2s, used);
            }
            return false;
        }
        for end in si + 1..=sb.len() {
            let sub = String::from_utf8(sb[si..end].to_vec()).unwrap();
            if used.contains_key(&sub) {
                continue;
            }
            p2s.insert(c, sub.clone());
            used.insert(sub.clone(), c);
            if dfs(pi + 1, end, p, sb, p2s, used) {
                return true;
            }
            used.remove(&sub);
            p2s.remove(&c);
        }
        false
    }

    dfs(0, 0, &p, &sb, &mut p2s, &mut used)
}

fn main() {
    println!("{}", word_pattern_match("abab".into(), "redblueredblue".into()));
}

#[cfg(test)]
mod tests {
    use super::word_pattern_match;

    #[test]
    fn example_one() {
        assert!(word_pattern_match("abab".into(), "redblueredblue".into()));
    }

    #[test]
    fn example_two() {
        assert!(!word_pattern_match("ab".into(), "aa".into()));
    }
}
