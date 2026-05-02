/// LeetCode #192 - Word Frequency (Bash)
pub const SCRIPT: &str = r#"cat words.txt | tr -s ' ' '\n' | sort | uniq -c | sort -nr | awk '{print $2, $1}'"#;

fn main() {
    println!("{}", SCRIPT.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SCRIPT;

    #[test]
    fn uses_sort_uniq() {
        assert!(SCRIPT.contains("uniq"));
    }
}
