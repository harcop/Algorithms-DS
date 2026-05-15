/// LeetCode #784 - Letter Case Permutation
fn letter_case_permutation(s: String) -> Vec<String> {
    let mut out = vec![String::new()];
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            let mut nxt = Vec::with_capacity(out.len() * 2);
            for t in out {
                nxt.push(format!("{}{}", t, c.to_ascii_lowercase()));
                nxt.push(format!("{}{}", t, c.to_ascii_uppercase()));
            }
            out = nxt;
        } else {
            for t in &mut out {
                t.push(c);
            }
        }
    }
    out
}

fn main() {
    println!("{:?}", letter_case_permutation("a1b2".into()));
}

#[cfg(test)]
mod tests {
    use super::letter_case_permutation;

    #[test]
    fn example_one() {
        let mut v = letter_case_permutation("a1b2".into());
        v.sort();
        let mut e = vec!["a1b2", "a1B2", "A1b2", "A1B2"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        e.sort();
        assert_eq!(v, e);
    }
}
