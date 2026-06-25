/// LeetCode #2072 - The Winner University (MySQL)
pub const SQL: &str = r#"SELECT
    CASE
        WHEN ny.excellent > ca.excellent THEN 'New York University'
        WHEN ny.excellent < ca.excellent THEN 'California University'
        ELSE 'No Winner'
    END AS winner
FROM
    (SELECT COUNT(*) AS excellent FROM NewYork WHERE score >= 90) AS ny,
    (SELECT COUNT(*) AS excellent FROM California WHERE score >= 90) AS ca"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn counts_excellent_students() {
        assert!(SQL.contains("score >= 90"));
    }
}
