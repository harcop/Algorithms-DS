/// LeetCode #2688 - Find Active Users (MySQL)
pub const SQL: &str = r#"SELECT DISTINCT
    user_id
FROM Users
WHERE
    user_id IN (
        SELECT
            user_id
        FROM
            (
                SELECT
                    user_id,
                    created_at,
                    LAG(created_at, 1) OVER (
                        PARTITION BY user_id
                        ORDER BY created_at
                    ) AS prev_created_at
                FROM Users
            ) AS t
        WHERE DATEDIFF(created_at, prev_created_at) <= 7
    )"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn uses_lag_and_datediff() {
        let upper = SQL.to_uppercase();
        assert!(upper.contains("LAG"));
        assert!(upper.contains("DATEDIFF"));
        assert!(upper.contains("PARTITION BY"));
        assert!(upper.contains("USER_ID"));
    }
}
