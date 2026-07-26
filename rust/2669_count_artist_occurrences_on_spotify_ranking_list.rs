/// LeetCode #2669 - Count Artist Occurrences On Spotify Ranking List (MySQL)
pub const SQL: &str = r#"SELECT
    artist,
    COUNT(1) AS occurrences
FROM Spotify
GROUP BY artist
ORDER BY occurrences DESC, artist"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn groups_and_orders() {
        let upper = SQL.to_uppercase();
        assert!(upper.contains("COUNT"));
        assert!(upper.contains("GROUP BY"));
        assert!(upper.contains("ORDER BY"));
    }
}
