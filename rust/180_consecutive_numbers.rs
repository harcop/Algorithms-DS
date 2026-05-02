/// LeetCode #180 - Consecutive Numbers (MySQL)
pub const SQL: &str = r#"SELECT DISTINCT l1.Num AS ConsecutiveNums
FROM Logs l1
JOIN Logs l2 ON l1.Id = l2.Id - 1 AND l1.Num = l2.Num
JOIN Logs l3 ON l2.Id = l3.Id - 1 AND l2.Num = l3.Num"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn joins_three() {
        assert!(SQL.matches("JOIN").count() >= 2);
    }
}
