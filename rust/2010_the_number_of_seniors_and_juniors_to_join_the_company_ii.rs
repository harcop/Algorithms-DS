/// LeetCode #2010 - The Number of Seniors and Juniors to Join the Company II (MySQL)
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
SELECT employee_id
FROM s
WHERE cur <= 70000
UNION
SELECT employee_id
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
