/// LeetCode #557 - Reverse Words in a String III
fn reverse_words(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut start = 0usize;
    for i in 0..=chars.len() {
        if i == chars.len() || chars[i] == ' ' {
            chars[start..i].reverse();
            start = i + 1;
        }
    }
    chars.into_iter().collect()
}

fn main() {
    println!("{}", reverse_words("Let's take LeetCode contest".into()));
}

#[cfg(test)]
mod tests {
    use super::reverse_words;

    #[test]
    fn example_one() {
        assert_eq!(
            reverse_words("Let's take LeetCode contest".into()),
            "s'teL ekat edoCteeL tsetnoc"
        );
    }
}
