/// LeetCode #2253 - Dynamic Unpivoting of a Table (MySQL)
pub const SQL: &str = r#"CREATE PROCEDURE UnpivotProducts()
BEGIN
    SET group_concat_max_len = 5000;
    WITH
        t AS (
            SELECT column_name
            FROM information_schema.columns
            WHERE
                table_schema = DATABASE()
                AND table_name = 'Products'
                AND column_name != 'product_id'
        )
    SELECT
        GROUP_CONCAT(
            'SELECT product_id, ''',
            column_name,
            ''' store, ',
            column_name,
            ' price FROM Products WHERE ',
            column_name,
            ' IS NOT NULL' SEPARATOR ' UNION '
        ) INTO @sql
    FROM t;
    PREPARE stmt FROM @sql;
    EXECUTE stmt;
    DEALLOCATE PREPARE stmt;
END"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn unpivots_products_by_store() {
        let sql = SQL.to_uppercase();
        assert!(sql.contains("CREATE PROCEDURE UNPIVOTPRODUCTS"));
        assert!(sql.contains("INFORMATION_SCHEMA.COLUMNS"));
        assert!(sql.contains("GROUP_CONCAT"));
        assert!(sql.contains("UNION"));
        assert!(sql.contains("PREPARE STMT"));
    }
}
