/// LeetCode #2142 - The Number of Passengers in Each Bus I (MySQL)
pub const SQL: &str = r#"SELECT
    bus_id,
    SUM(passengers_change) OVER (ORDER BY bus_id) AS passengers
FROM Buses
ORDER BY bus_id"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn uses_running_sum() {
        assert!(SQL.to_uppercase().contains("SUM"));
        assert!(SQL.to_uppercase().contains("OVER"));
    }

    #[test]
    fn orders_by_bus_id() {
        assert!(SQL.to_uppercase().contains("ORDER BY BUS_ID"));
    }
}
