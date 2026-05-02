/// LeetCode #178 - Rank Scores (MySQL)
pub const SQL: &str = r#"SELECT s.Score,
  (SELECT COUNT(DISTINCT t.Score) FROM Scores t WHERE t.Score >= s.Score) AS `Rank`
FROM Scores s
ORDER BY s.Score DESC"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn has_rank() {
        assert!(SQL.contains("Rank"));
    }
}
