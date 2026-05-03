/// LeetCode #197 - Rising Temperature (MySQL)
pub const SQL: &str = r#"SELECT w1.Id
FROM Weather w1
JOIN Weather w2 ON DATEDIFF(w1.RecordDate, w2.RecordDate) = 1 AND w1.Temperature > w2.Temperature"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn joins_weather() {
        assert!(SQL.to_lowercase().contains("weather"));
    }
}
