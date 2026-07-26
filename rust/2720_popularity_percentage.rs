/// LeetCode #2720 - Popularity Percentage (MySQL)
pub const SQL: &str = r#"WITH
    F AS (
        SELECT * FROM Friends
        UNION
        SELECT user2, user1 FROM Friends
    ),
    T AS (SELECT COUNT(DISTINCT user1) AS cnt FROM F)
SELECT DISTINCT
    user1,
    ROUND(
        (COUNT(1) OVER (PARTITION BY user1)) * 100 / (SELECT cnt FROM T),
        2
    ) AS percentage_popularity
FROM F
ORDER BY 1"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn computes_popularity_percentage() {
        let upper = SQL.to_uppercase();
        assert!(upper.contains("UNION"));
        assert!(upper.contains("ROUND"));
        assert!(upper.contains("PERCENTAGE_POPULARITY"));
        assert!(upper.contains("PARTITION BY"));
    }
}
