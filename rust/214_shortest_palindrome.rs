/// LeetCode #214 - Shortest Palindrome
fn shortest_palindrome(s: String) -> String {
    if s.is_empty() {
        return s;
    }
    let rev: String = s.chars().rev().collect();
    let combined = format!("{}#{}", s, rev);
    let b = combined.as_bytes();
    let mut lps = vec![0usize; b.len()];
    let mut j = 0usize;
    for i in 1..b.len() {
        while j > 0 && b[i] != b[j] {
            j = lps[j - 1];
        }
        if b[i] == b[j] {
            j += 1;
        }
        lps[i] = j;
    }
    let k = lps[b.len() - 1];
    format!("{}{}", &rev[..s.len() - k], s)
}

fn main() {
    println!("{}", shortest_palindrome("aacecaaa".into()));
}

#[cfg(test)]
mod tests {
    use super::shortest_palindrome;

    #[test]
    fn example_one() {
        assert_eq!(shortest_palindrome("aacecaaa".into()), "aaacecaaa");
    }

    #[test]
    fn example_two() {
        assert_eq!(shortest_palindrome("abcd".into()), "dcbabcd");
    }
}
