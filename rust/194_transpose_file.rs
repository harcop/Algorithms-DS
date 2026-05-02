/// LeetCode #194 - Transpose File (Bash)
pub const SCRIPT: &str = r#"awk '
{
  for (i = 1; i <= NF; i++) col[i] = col[i] $i (NR > 1 ? " " : "") (FNR == NR ? "" : "")
}
END {
  for (i = 1; i <= NF; i++) print col[i]
}' file.txt"#;

fn main() {
    println!("{}", SCRIPT.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SCRIPT;

    #[test]
    fn mentions_awk() {
        assert!(SCRIPT.contains("awk"));
    }
}
