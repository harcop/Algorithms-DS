/// LeetCode #844 - Backspace String Compare
fn backspace_compare(s: String, t: String) -> bool {
    fn build(s: &str) -> String {
        let mut st = Vec::new();
        for c in s.chars() {
            if c == '#' {
                st.pop();
            } else {
                st.push(c);
            }
        }
        st.into_iter().collect()
    }
    build(&s) == build(&t)
}

fn main() {
    println!("{}", backspace_compare("ab#c".into(), "ad#c".into()));
}

#[cfg(test)]
mod tests {
    use super::backspace_compare;

    #[test]
    fn example_one() {
        assert!(backspace_compare("ab#c".into(), "ad#c".into()));
    }

    #[test]
    fn example_two() {
        assert!(backspace_compare("ab##".into(), "c#d#".into()));
    }
}
