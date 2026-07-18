/// LeetCode #2480 - Form a Chemical Bond (MySQL)
pub const SQL: &str = r#"SELECT a.symbol AS metal, b.symbol AS nonmetal
FROM
    Elements AS a,
    Elements AS b
WHERE a.type = 'Metal' AND b.type = 'Nonmetal'"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn pairs_metal_and_nonmetal() {
        assert!(SQL.contains("Metal"));
        assert!(SQL.contains("Nonmetal"));
    }
}
