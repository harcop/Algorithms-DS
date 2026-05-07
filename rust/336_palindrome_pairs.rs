/// LeetCode #336 - Palindrome Pairs (O(n²·L); clear for correctness)
fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
    let wb: Vec<&[u8]> = words.iter().map(|s| s.as_bytes()).collect();
    let mut out = vec![];
    for i in 0..words.len() {
        for j in 0..words.len() {
            if i == j {
                continue;
            }
            let mut cat = Vec::with_capacity(words[i].len() + words[j].len());
            cat.extend_from_slice(wb[i]);
            cat.extend_from_slice(wb[j]);
            if is_palindrome_bytes(&cat) {
                out.push(vec![i as i32, j as i32]);
            }
        }
    }
    out
}

fn is_palindrome_bytes(w: &[u8]) -> bool {
    let mut l = 0usize;
    let mut r = w.len();
    while l < r {
        r -= 1;
        if w[l] != w[r] {
            return false;
        }
        l += 1;
    }
    true
}

fn main() {
    println!(
        "{:?}",
        palindrome_pairs(vec!["abcd".into(), "dcba".into(), "lls".into(), "s".into(), "sssll".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::palindrome_pairs;

    #[test]
    fn example() {
        let mut got = palindrome_pairs(vec![
            "abcd".into(),
            "dcba".into(),
            "lls".into(),
            "s".into(),
            "sssll".into(),
        ]);
        got.sort_unstable();
        let mut exp = vec![vec![0, 1], vec![1, 0], vec![3, 2], vec![2, 4]];
        exp.sort_unstable();
        assert_eq!(got, exp);
    }

    #[test]
    fn with_palindrome() {
        let got = palindrome_pairs(vec!["aba".into(), "".into()]);
        assert!(got.contains(&vec![0, 1]) && got.contains(&vec![1, 0]));
    }
}
