/// LeetCode #181 - Employees Earning More Than Their Managers (MySQL)
pub const SQL: &str = r#"SELECT e.Name AS Employee
FROM Employee e
JOIN Employee m ON e.ManagerId = m.Id
WHERE e.Salary > m.Salary"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn compares_salary() {
        assert!(SQL.contains("Salary"));
    }
}
