/// LeetCode #2687 - Bikes Last Time Used (MySQL)
pub const SQL: &str = r#"SELECT
    bike_number,
    MAX(end_time) AS end_time
FROM Bikes
GROUP BY bike_number
ORDER BY end_time DESC"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn uses_max_group_by_and_order() {
        let upper = SQL.to_uppercase();
        assert!(upper.contains("MAX"));
        assert!(upper.contains("GROUP BY"));
        assert!(upper.contains("ORDER BY"));
        assert!(upper.contains("END_TIME"));
        assert!(upper.contains("DESC"));
    }
}
