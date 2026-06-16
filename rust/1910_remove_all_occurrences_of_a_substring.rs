/// LeetCode #1910 - Remove All Occurrences of a Substring
fn remove_occurrences(mut s: String, part: String) -> String {
    while let Some(pos) = s.find(&part) {
        s.replace_range(pos..pos + part.len(), "");
    }
    s
}

fn main() {
    println!("{}", remove_occurrences("daabcbaabcbc".into(), "abc".into()));
}

#[cfg(test)]
mod tests {
    use super::remove_occurrences;

    #[test]
    fn example_one() {
        assert_eq!(
            remove_occurrences("daabcbaabcbc".into(), "abc".into()),
            "dab"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(remove_occurrences("axxxxyyyyb".into(), "xy".into()), "ab");
    }
}
