/// LeetCode #193 - Valid Phone Numbers (Bash)
pub const SCRIPT: &str = r#"grep -E '^(\([0-9]{3}\) [0-9]{3}-[0-9]{4}|[0-9]{3}-[0-9]{3}-[0-9]{4})$' file.txt"#;

fn main() {
    println!("{}", SCRIPT.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SCRIPT;

    #[test]
    fn uses_grep() {
        assert!(SCRIPT.contains("grep"));
    }
}
