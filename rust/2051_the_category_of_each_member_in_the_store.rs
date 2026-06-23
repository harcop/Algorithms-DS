/// LeetCode #2051 - The Category of Each Member in the Store (MySQL)
pub const SQL: &str = r#"SELECT
    m.member_id,
    name,
    CASE
        WHEN COUNT(v.visit_id) = 0 THEN 'Bronze'
        WHEN 100 * COUNT(charged_amount) / COUNT(v.visit_id) >= 80 THEN 'Diamond'
        WHEN 100 * COUNT(charged_amount) / COUNT(v.visit_id) >= 50 THEN 'Gold'
        ELSE 'Silver'
    END AS category
FROM
    Members AS m
    LEFT JOIN Visits AS v ON m.member_id = v.member_id
    LEFT JOIN Purchases AS p ON v.visit_id = p.visit_id
GROUP BY member_id"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn uses_case() {
        assert!(SQL.to_uppercase().contains("CASE"));
    }
}
