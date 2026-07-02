/// LeetCode #2205 - The Number of Users That Are Eligible for Discount (MySQL)
pub const SQL: &str = r#"CREATE FUNCTION getUserIDs(startDate DATE, endDate DATE, minAmount INT) RETURNS INT
BEGIN
  RETURN (
      SELECT COUNT(DISTINCT user_id) AS user_cnt
      FROM Purchases
      WHERE time_stamp BETWEEN startDate AND endDate AND amount >= minAmount
  );
END"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn counts_eligible_users() {
        let sql = SQL.to_uppercase();
        assert!(sql.contains("COUNT(DISTINCT USER_ID)"));
        assert!(sql.contains("BETWEEN STARTDATE AND ENDDATE"));
        assert!(sql.contains("AMOUNT >= MINAMOUNT"));
    }
}
