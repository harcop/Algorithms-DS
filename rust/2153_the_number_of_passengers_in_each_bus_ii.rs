/// LeetCode #2153 - The Number of Passengers in Each Bus II (MySQL)
pub const SQL: &str = r#"WITH RECURSIVE
ordered_buses AS (
    SELECT
        bus_id,
        arrival_time,
        capacity,
        ROW_NUMBER() OVER (ORDER BY arrival_time) AS rn
    FROM Buses
),
new_passengers AS (
    SELECT
        b.rn,
        COUNT(p.passenger_id) AS cnt
    FROM ordered_buses AS b
    LEFT JOIN ordered_buses AS prev ON prev.rn = b.rn - 1
    LEFT JOIN Passengers AS p
        ON p.arrival_time <= b.arrival_time
        AND (prev.arrival_time IS NULL OR p.arrival_time > prev.arrival_time)
    GROUP BY b.rn
),
boarded AS (
    SELECT
        b.rn,
        b.bus_id,
        LEAST(b.capacity, p.cnt) AS passengers_cnt,
        p.cnt - LEAST(b.capacity, p.cnt) AS waiting
    FROM ordered_buses AS b
    JOIN new_passengers AS p ON b.rn = p.rn
    WHERE b.rn = 1

    UNION ALL

    SELECT
        b.rn,
        b.bus_id,
        LEAST(b.capacity, p.cnt + boarded.waiting) AS passengers_cnt,
        p.cnt + boarded.waiting - LEAST(b.capacity, p.cnt + boarded.waiting) AS waiting
    FROM boarded
    JOIN ordered_buses AS b ON b.rn = boarded.rn + 1
    JOIN new_passengers AS p ON b.rn = p.rn
)
SELECT bus_id, passengers_cnt
FROM boarded
ORDER BY bus_id"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn uses_recursive_boarding() {
        let sql = SQL.to_uppercase();
        assert!(sql.contains("WITH RECURSIVE"));
        assert!(sql.contains("LEAST"));
    }
}
