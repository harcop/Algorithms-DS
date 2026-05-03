/// LeetCode #195 - Tenth Line (Bash)
pub const SCRIPT: &str = r#"sed -n '10p' file.txt"#;

fn main() {
    println!("{}", SCRIPT);
}

#[cfg(test)]
mod tests {
    use super::SCRIPT;

    #[test]
    fn uses_sed() {
        assert!(SCRIPT.contains("10"));
    }
}
