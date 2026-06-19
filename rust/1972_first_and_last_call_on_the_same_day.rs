/// LeetCode #1972 - First and Last Call On the Same Day (MySQL)
pub const SQL: &str = r#"WITH
    s AS (
        SELECT *
        FROM Calls
        UNION ALL
        SELECT recipient_id, caller_id, call_time
        FROM Calls
    ),
    t AS (
        SELECT
            caller_id AS user_id,
            FIRST_VALUE(recipient_id) OVER (
                PARTITION BY DATE_FORMAT(call_time, '%Y-%m-%d'), caller_id
                ORDER BY call_time ASC
            ) AS first,
            FIRST_VALUE(recipient_id) OVER (
                PARTITION BY DATE_FORMAT(call_time, '%Y-%m-%d'), caller_id
                ORDER BY call_time DESC
            ) AS last
        FROM s
    )
SELECT DISTINCT user_id
FROM t
WHERE first = last"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn uses_window_functions() {
        assert!(SQL.to_uppercase().contains("FIRST_VALUE"));
    }
}
