use std::collections::HashMap;

/// LeetCode #87 - Scramble String
fn is_scramble(s1: String, s2: String) -> bool {
    let a = s1.into_bytes();
    let b = s2.into_bytes();
    let mut memo = HashMap::new();
    dfs(0, 0, a.len(), &a, &b, &mut memo)
}

fn dfs(
    i: usize,
    j: usize,
    len: usize,
    a: &[u8],
    b: &[u8],
    memo: &mut HashMap<(usize, usize, usize), bool>,
) -> bool {
    let key = (i, j, len);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    if &a[i..i + len] == &b[j..j + len] {
        memo.insert(key, true);
        return true;
    }
    let mut cnt = [0i32; 26];
    for k in 0..len {
        cnt[(a[i + k] - b'a') as usize] += 1;
        cnt[(b[j + k] - b'a') as usize] -= 1;
    }
    if cnt.iter().any(|&x| x != 0) {
        memo.insert(key, false);
        return false;
    }
    for k in 1..len {
        if dfs(i, j, k, a, b, memo) && dfs(i + k, j + k, len - k, a, b, memo) {
            memo.insert(key, true);
            return true;
        }
        if dfs(i, j + len - k, k, a, b, memo) && dfs(i + k, j, len - k, a, b, memo) {
            memo.insert(key, true);
            return true;
        }
    }
    memo.insert(key, false);
    false
}

fn main() {
    println!("{}", is_scramble("great".to_string(), "rgeat".to_string()));
}

#[cfg(test)]
mod tests {
    use super::is_scramble;

    #[test]
    fn example_one() {
        assert!(is_scramble("great".to_string(), "rgeat".to_string()));
    }

    #[test]
    fn example_two() {
        assert!(!is_scramble("abcde".to_string(), "caebd".to_string()));
    }

    #[test]
    fn example_three() {
        assert!(is_scramble("a".to_string(), "a".to_string()));
    }
}
