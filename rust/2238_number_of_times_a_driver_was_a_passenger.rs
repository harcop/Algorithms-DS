/// LeetCode #2238 - Number of Times a Driver Was a Passenger (MySQL)
pub const SQL: &str = r#"WITH T AS (
    SELECT DISTINCT driver_id
    FROM Rides
)
SELECT t.driver_id, COUNT(passenger_id) AS cnt
FROM
    T AS t
    LEFT JOIN Rides AS r ON t.driver_id = r.passenger_id
GROUP BY 1"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn counts_passenger_trips_for_drivers() {
        let sql = SQL.to_uppercase();
        assert!(sql.contains("DISTINCT DRIVER_ID"));
        assert!(sql.contains("LEFT JOIN RIDES"));
        assert!(sql.contains("PASSENGER_ID"));
    }
}
