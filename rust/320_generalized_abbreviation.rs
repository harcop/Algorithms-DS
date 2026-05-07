/// LeetCode #320 - Generalized Abbreviation
fn generate_abbreviations(word: String) -> Vec<String> {
    let w: Vec<u8> = word.into_bytes();
    let n = w.len();
    let mut out = Vec::new();
    for mask in 0u32..(1u32 << n) {
        let mut s = String::new();
        let mut i = 0usize;
        while i < n {
            if (mask >> i) & 1 == 0 {
                s.push(w[i] as char);
                i += 1;
            } else {
                let mut j = i;
                while j < n && (mask >> j) & 1 == 1 {
                    j += 1;
                }
                s.push_str(&(j - i).to_string());
                i = j;
            }
        }
        out.push(s);
    }
    out
}

fn main() {
    println!("{:?}", generate_abbreviations("word".into()));
}

#[cfg(test)]
mod tests {
    use super::generate_abbreviations;

    #[test]
    fn example_word() {
        let mut v = generate_abbreviations("word".into());
        v.sort();
        assert!(v.contains(&"word".into()));
        assert!(v.contains(&"4".into()));
        assert_eq!(v.len(), 16);
    }
}
