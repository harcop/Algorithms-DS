/// LeetCode #2252 - Dynamic Pivoting of a Table (MySQL)
pub const SQL: &str = r#"CREATE PROCEDURE PivotProducts()
BEGIN
    SET group_concat_max_len = 5000;
    SELECT GROUP_CONCAT(DISTINCT 'MAX(CASE WHEN store = ''',
               store,
               ''' THEN price ELSE NULL END) AS `',
               store,
               '`'
               ORDER BY store) INTO @sql
    FROM Products;
    SET @sql = CONCAT('SELECT product_id, ', @sql, ' FROM Products GROUP BY product_id');
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
    fn pivots_products_by_store() {
        let sql = SQL.to_uppercase();
        assert!(sql.contains("CREATE PROCEDURE PIVOTPRODUCTS"));
        assert!(sql.contains("GROUP_CONCAT"));
        assert!(sql.contains("MAX(CASE WHEN STORE"));
        assert!(sql.contains("GROUP BY PRODUCT_ID"));
        assert!(sql.contains("PREPARE STMT"));
    }
}
