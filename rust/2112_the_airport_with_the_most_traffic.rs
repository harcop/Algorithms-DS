/// LeetCode #2112 - The Airport With the Most Traffic (MySQL)
pub const SQL: &str = r#"WITH airport_traffic AS (
    SELECT departure_airport AS airport_id, flights_count
    FROM Flights
    UNION ALL
    SELECT arrival_airport AS airport_id, flights_count
    FROM Flights
),
totals AS (
    SELECT airport_id, SUM(flights_count) AS total_flights
    FROM airport_traffic
    GROUP BY airport_id
)
SELECT airport_id
FROM totals
WHERE total_flights = (SELECT MAX(total_flights) FROM totals)"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn combines_departures_and_arrivals() {
        assert!(SQL.contains("departure_airport"));
        assert!(SQL.contains("arrival_airport"));
        assert!(SQL.to_uppercase().contains("UNION ALL"));
    }

    #[test]
    fn filters_to_maximum_traffic() {
        assert!(SQL.to_uppercase().contains("MAX"));
    }
}
