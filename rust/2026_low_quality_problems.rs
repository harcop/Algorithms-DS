/// LeetCode #2026 - Low-Quality Problems (MySQL)
pub const SQL: &str = r#"SELECT problem_id
FROM Problems
WHERE likes / (likes + dislikes) < 0.6
ORDER BY problem_id"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn filters_by_ratio() {
        assert!(SQL.contains("0.6"));
    }
}
