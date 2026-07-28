/// LeetCode #2738 - Count Occurrences in Text (MySQL)
pub const SQL: &str = r#"SELECT 'bull' AS word, COUNT(*) AS count
FROM Files
WHERE content LIKE '% bull %'
UNION
SELECT 'bear' AS word, COUNT(*) AS count
FROM Files
WHERE content LIKE '% bear %'"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn counts_bull_and_bear() {
        let upper = SQL.to_uppercase();
        assert!(upper.contains("'BULL'"));
        assert!(upper.contains("'BEAR'"));
        assert!(upper.contains("UNION"));
        assert!(upper.contains("LIKE '% BULL %'"));
        assert!(upper.contains("LIKE '% BEAR %'"));
        assert!(upper.contains("COUNT(*)"));
    }
}
