/// LeetCode #176 - Second Highest Salary (MySQL)
pub const SQL: &str = r#"SELECT MAX(Salary) AS SecondHighestSalary
FROM Employee
WHERE Salary < (SELECT MAX(Salary) FROM Employee)"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn references_salary() {
        assert!(SQL.to_lowercase().contains("salary"));
    }
}
