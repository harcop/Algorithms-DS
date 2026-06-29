/// LeetCode #2159 - Order Two Columns Independently (MySQL)
pub const SQL: &str = r#"WITH
ordered_first AS (
    SELECT
        first_col,
        ROW_NUMBER() OVER (ORDER BY first_col) AS rn
    FROM Data
),
ordered_second AS (
    SELECT
        second_col,
        ROW_NUMBER() OVER (ORDER BY second_col DESC) AS rn
    FROM Data
)
SELECT first_col, second_col
FROM ordered_first
JOIN ordered_second USING (rn)"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn orders_columns_with_row_numbers() {
        let sql = SQL.to_uppercase();
        assert!(sql.contains("ROW_NUMBER"));
        assert!(sql.contains("ORDER BY SECOND_COL DESC"));
    }
}
