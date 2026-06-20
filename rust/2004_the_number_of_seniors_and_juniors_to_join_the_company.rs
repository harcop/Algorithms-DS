/// LeetCode #2004 - The Number of Seniors and Juniors to Join the Company (MySQL)
pub const SQL: &str = r#"WITH
    s AS (
        SELECT
            employee_id,
            SUM(salary) OVER (ORDER BY salary) AS cur
        FROM Candidates
        WHERE experience = 'Senior'
    ),
    j AS (
        SELECT
            employee_id,
            IFNULL(
                (
                    SELECT MAX(cur)
                    FROM s
                    WHERE cur <= 70000
                ),
                0
            ) + SUM(salary) OVER (ORDER BY salary) AS cur
        FROM Candidates
        WHERE experience = 'Junior'
    )
SELECT 'Senior' AS experience, COUNT(employee_id) AS accepted_candidates
FROM s
WHERE cur <= 70000
UNION ALL
SELECT 'Junior' AS experience, COUNT(employee_id) AS accepted_candidates
FROM j
WHERE cur <= 70000"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn uses_window_functions() {
        assert!(SQL.to_uppercase().contains("SUM(SALARY) OVER"));
    }
}
