/// LeetCode #262 - Trips and Users (MySQL)
pub const SQL: &str = r#"SELECT t.Request_at AS Day,
  ROUND(SUM(IF(t.Status != 'completed', 1, 0)) / COUNT(*), 2) AS `Cancellation Rate`
FROM Trips t
JOIN Users u ON t.Client_Id = u.Users_Id AND u.Banned = 'No'
WHERE t.Request_at BETWEEN '2013-10-01' AND '2013-10-03'
GROUP BY t.Request_at"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn has_trips() {
        assert!(SQL.to_lowercase().contains("trips"));
    }
}
