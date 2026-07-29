/// LeetCode #2800 - Shortest String That Contains Three Strings
fn merge_two(s: &str, t: &str) -> String {
    if s.contains(t) {
        return s.to_string();
    }
    if t.contains(s) {
        return t.to_string();
    }
    let (m, n) = (s.len(), t.len());
    for i in (1..=m.min(n)).rev() {
        if s[m - i..] == t[..i] {
            return format!("{}{}", s, &t[i..]);
        }
    }
    format!("{s}{t}")
}

fn minimum_string(a: &str, b: &str, c: &str) -> String {
    let s = [a, b, c];
    let perm = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    let mut ans = String::new();
    for [i, j, k] in perm {
        let t = merge_two(&merge_two(s[i], s[j]), s[k]);
        if ans.is_empty() || t.len() < ans.len() || (t.len() == ans.len() && t < ans) {
            ans = t;
        }
    }
    ans
}

fn main() {
    println!("{}", minimum_string("abc", "bca", "aaa"));
}

#[cfg(test)]
mod tests {
    use super::minimum_string;

    #[test]
    fn example_one() {
        assert_eq!(minimum_string("abc", "bca", "aaa"), "aaabca");
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_string("ab", "ba", "aba"), "aba");
    }
}
