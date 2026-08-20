/// LeetCode #3324 - Find the Sequence of Strings Appeared on the Screen
fn string_sequence(target: String) -> Vec<String> {
    let mut ans: Vec<String> = Vec::new();
    for c in target.chars() {
        let s = ans.last().cloned().unwrap_or_default();
        for a in 'a'..=c {
            ans.push(format!("{s}{a}"));
        }
    }
    ans
}

fn main() {
    println!("{:?}", string_sequence("abc".into()));
}

#[cfg(test)]
mod tests {
    use super::string_sequence;

    #[test]
    fn example1() {
        assert_eq!(
            string_sequence("abc".into()),
            vec!["a", "aa", "ab", "aba", "abb", "abc"]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            string_sequence("he".into()),
            vec![
                "a", "b", "c", "d", "e", "f", "g", "h", "ha", "hb", "hc", "hd", "he"
            ]
        );
    }
}
