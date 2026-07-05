/// LeetCode #2230 - The Users That Are Eligible for Discount (MySQL)
pub const SQL: &str = r#"CREATE PROCEDURE getUserIDs(startDate DATE, endDate DATE, minAmount INT)
BEGIN
  SELECT DISTINCT user_id
  FROM Purchases
  WHERE
    time_stamp BETWEEN startDate AND endDate
    AND amount >= minAmount
  ORDER BY 1;
END"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn selects_eligible_users() {
        let sql = SQL.to_uppercase();
        assert!(sql.contains("SELECT DISTINCT USER_ID"));
        assert!(sql.contains("BETWEEN STARTDATE AND ENDDATE"));
        assert!(sql.contains("AMOUNT >= MINAMOUNT"));
        assert!(sql.contains("ORDER BY 1"));
    }
}
