/// LeetCode #187 - Repeated DNA Sequences
use std::collections::HashSet;

fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    let b = s.as_bytes();
    let mut seen = HashSet::new();
    let mut twice = HashSet::new();
    if b.len() < 10 {
        return vec![];
    }
    for i in 0..=b.len() - 10 {
        let key = std::str::from_utf8(&b[i..i + 10]).unwrap();
        if !seen.insert(key) {
            twice.insert(key.to_string());
        }
    }
    let mut out: Vec<String> = twice.into_iter().collect();
    out.sort();
    out
}

fn main() {
    println!("{:?}", find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".into()));
}

#[cfg(test)]
mod tests {
    use super::find_repeated_dna_sequences;

    #[test]
    fn example_one() {
        let mut v = find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".into());
        v.sort();
        let mut e = vec!["AAAAACCCCC".to_string(), "CCCCCAAAAA".to_string()];
        e.sort();
        assert_eq!(v, e);
    }

    #[test]
    fn example_two() {
        let mut v = find_repeated_dna_sequences("AAAAAAAAAAAAA".into());
        v.sort();
        assert_eq!(v, vec!["AAAAAAAAAA".to_string()]);
    }
}
