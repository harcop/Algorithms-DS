/// LeetCode #1307 - Verbal Arithmetic Puzzle
fn is_solvable(words: Vec<String>, result: String) -> bool {
    let mut letters = vec![];
    for w in words.iter().chain(std::iter::once(&result)) {
        for c in w.chars() {
            if c.is_ascii_alphabetic() {
                letters.push(c);
            }
        }
    }
    letters.sort_unstable();
    letters.dedup();
    if letters.len() > 10 {
        return false;
    }
    let mut lead = std::collections::HashSet::new();
    for w in words.iter().chain(std::iter::once(&result)) {
        if w.len() > 1 {
            lead.insert(w.as_bytes()[0] as char);
        }
    }
    let mut perm = (0..10).collect::<Vec<_>>();
    fn dfs(
        idx: usize,
        letters: &[char],
        perm: &mut [i32],
        lead: &std::collections::HashSet<char>,
        words: &[String],
        result: &str,
    ) -> bool {
        if idx == letters.len() {
            let mut map = std::collections::HashMap::new();
            for (i, &c) in letters.iter().enumerate() {
                map.insert(c, perm[i]);
            }
            for &c in lead.iter() {
                if map.get(&c) == Some(&0) {
                    return false;
                }
            }
            let val = |s: &str| -> i64 {
                let mut x = 0i64;
                for c in s.chars() {
                    x = x * 10 + *map.get(&c).unwrap_or(&0) as i64;
                }
                x
            };
            let sum: i64 = words.iter().map(|w| val(w)).sum();
            return sum == val(result);
        }
        for j in idx..10 {
            perm.swap(idx, j);
            if dfs(idx + 1, letters, perm, lead, words, result) {
                return true;
            }
            perm.swap(idx, j);
        }
        false
    }
    dfs(0, &letters, &mut perm, &lead, &words, &result)
}

fn main() {
    println!("{}", is_solvable(vec!["SEND".to_string(), "MORE".to_string()], "MONEY".to_string()));
}

#[cfg(test)]
mod tests {
    use super::is_solvable;

    #[test]
    fn example_one() {
        assert!(is_solvable(vec!["SEND".to_string(), "MORE".to_string()], "MONEY".to_string()));
    }

    #[test]
    fn example_two() {
        assert!(is_solvable(vec!["SEND".to_string(), "MORE".to_string()], "MONEY".to_string()));
    }
}
