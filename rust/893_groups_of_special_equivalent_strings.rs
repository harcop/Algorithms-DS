/// LeetCode #893 - Groups of Special-Equivalent Strings
use std::collections::HashSet;

fn num_special_equiv_groups(words: Vec<String>) -> i32 {
    let mut seen = HashSet::new();
    for w in words {
        let mut ev: Vec<char> = w
            .chars()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, c)| c)
            .collect();
        let mut od: Vec<char> = w
            .chars()
            .enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .map(|(_, c)| c)
            .collect();
        ev.sort_unstable();
        od.sort_unstable();
        let mut key = String::new();
        key.extend(ev);
        key.push('|');
        key.extend(od);
        seen.insert(key);
    }
    seen.len() as i32
}

fn main() {
    println!(
        "{}",
        num_special_equiv_groups(vec!["abcd".into(), "cdab".into(), "cbad".into(), "xyzz".into(), "zzxy".into(), "zzyx".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::num_special_equiv_groups;

    #[test]
    fn example_one() {
        let w: Vec<String> = vec!["abcd", "cdab", "cbad", "xyzz", "zzxy", "zzyx"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(num_special_equiv_groups(w), 3);
    }
}
