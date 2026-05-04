/// LeetCode #267 - Palindrome Permutation II
use std::collections::HashMap;

fn generate_palindromes(s: String) -> Vec<String> {
    let mut freq: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }
    let mut odd: Option<char> = None;
    for (&c, &k) in &freq {
        if k % 2 == 1 {
            if odd.is_some() {
                return vec![];
            }
            odd = Some(c);
        }
    }
    let mut half = String::new();
    for (&c, &k) in &freq {
        let n = k / 2;
        for _ in 0..n {
            half.push(c);
        }
    }
    let mut chars: Vec<char> = half.chars().collect();
    chars.sort_unstable();
    let mut out = vec![];
    let mut used = vec![false; chars.len()];
    fn dfs(
        path: &mut Vec<char>,
        chars: &[char],
        used: &mut [bool],
        mid: Option<char>,
        out: &mut Vec<String>,
    ) {
        if path.len() == chars.len() {
            let left: String = path.iter().collect();
            let rev: String = path.iter().rev().collect();
            let m = mid.map(|c| c.to_string()).unwrap_or_default();
            out.push(format!("{}{}{}", left, m, rev));
            return;
        }
        for i in 0..chars.len() {
            if used[i] {
                continue;
            }
            if i > 0 && chars[i] == chars[i - 1] && !used[i - 1] {
                continue;
            }
            used[i] = true;
            path.push(chars[i]);
            dfs(path, chars, used, mid, out);
            path.pop();
            used[i] = false;
        }
    }
    let mut path = vec![];
    dfs(&mut path, &chars, &mut used, odd, &mut out);
    out
}

fn main() {
    println!("{:?}", generate_palindromes("aabb".into()));
}

#[cfg(test)]
mod tests {
    use super::generate_palindromes;

    #[test]
    fn example_one() {
        let mut v = generate_palindromes("aabb".into());
        v.sort();
        assert_eq!(v, vec!["abba", "baab"]);
    }

    #[test]
    fn example_two() {
        assert_eq!(generate_palindromes("abc".into()), Vec::<String>::new());
    }
}
