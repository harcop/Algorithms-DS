/// LeetCode #2825 - Make String a Subsequence Using Cyclic Increments
fn can_make_subsequence(source: String, target: String) -> bool {
    let s: Vec<char> = source.chars().collect();
    let t: Vec<char> = target.chars().collect();
    let mut i = 0usize;
    let mut j = 0usize;
    while i < s.len() && j < t.len() {
        let c = s[i];
        let need = t[j];
        let next = if c == 'z' { 'a' } else { ((c as u8) + 1) as char };
        if c == need || next == need {
            j += 1;
        }
        i += 1;
    }
    j == t.len()
}

fn main() {
    println!("{}", can_make_subsequence("abc".into(), "ad".into()));
}

#[cfg(test)]
mod tests {
    use super::can_make_subsequence;

    #[test]
    fn example_one() {
        assert!(can_make_subsequence("abc".into(), "ad".into()));
    }

    #[test]
    fn example_two() {
        assert!(can_make_subsequence("zc".into(), "ad".into()));
    }

    #[test]
    fn example_three() {
        assert!(!can_make_subsequence("ab".into(), "d".into()));
    }
}
