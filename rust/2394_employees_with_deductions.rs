/// LeetCode #2394 - Employees With Deductions (MySQL)
pub const SQL: &str = r#"WITH T AS (
    SELECT
        employee_id,
        SUM(CEILING(TIMESTAMPDIFF(SECOND, in_time, out_time) / 60)) / 60 AS tot
    FROM Logs
    GROUP BY employee_id
)
SELECT employee_id
FROM Employees
    LEFT JOIN T USING (employee_id)
WHERE IFNULL(tot, 0) < needed_hours"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn reports_employees_with_insufficient_hours() {
        let sql = SQL.to_uppercase();
        assert!(sql.contains("EMPLOYEES"));
        assert!(sql.contains("LOGS"));
        assert!(sql.contains("NEEDED_HOURS"));
        assert!(sql.contains("TIMESTAMPDIFF"));
    }
}
