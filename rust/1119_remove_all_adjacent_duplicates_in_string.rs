/// LeetCode #1119 - Remove All Adjacent Duplicates In String
fn remove_duplicates(s: String) -> String {
    let mut st: Vec<char> = Vec::new();
    for c in s.chars() {
        if st.last() == Some(&c) {
            st.pop();
        } else {
            st.push(c);
        }
    }
    st.into_iter().collect()
}

fn main() {
    println!("{}", remove_duplicates("abbaca".into()));
}

#[cfg(test)]
mod tests {
    use super::remove_duplicates;

    #[test]
    fn example_one() {
        assert_eq!(remove_duplicates("abbaca".into()), "ca");
    }

    #[test]
    fn example_two() {
        assert_eq!(remove_duplicates("azxxzy".into()), "ay");
    }
}
