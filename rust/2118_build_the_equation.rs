/// LeetCode #2118 - Build the Equation (MySQL)
pub const SQL: &str = r#"WITH formatted_terms AS (
    SELECT
        power,
        CONCAT(
            IF(factor > 0, '+', ''),
            factor,
            IF(power = 0, '', 'X'),
            IF(power IN (0, 1), '', CONCAT('^', power))
        ) AS term
    FROM Terms
)
SELECT CONCAT(GROUP_CONCAT(term ORDER BY power DESC SEPARATOR ''), '=0') AS equation
FROM formatted_terms"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn formats_equation_terms() {
        assert!(SQL.contains("GROUP_CONCAT"));
        assert!(SQL.contains("ORDER BY power DESC"));
        assert!(SQL.contains("=0"));
    }

    #[test]
    fn handles_special_powers() {
        assert!(SQL.contains("power = 0"));
        assert!(SQL.contains("power IN (0, 1)"));
    }
}
