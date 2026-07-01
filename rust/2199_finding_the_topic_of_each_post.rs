/// LeetCode #2199 - Finding the Topic of Each Post (MySQL)
pub const SQL: &str = r#"SELECT
    post_id,
    IFNULL(GROUP_CONCAT(DISTINCT topic_id), 'Ambiguous!') AS topic
FROM
    Posts
    LEFT JOIN Keywords ON INSTR(CONCAT(' ', content, ' '), CONCAT(' ', word, ' ')) > 0
GROUP BY post_id"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn joins_posts_with_keywords() {
        let sql = SQL.to_uppercase();
        assert!(sql.contains("GROUP_CONCAT"));
        assert!(sql.contains("AMBIGUOUS!"));
        assert!(sql.contains("INSTR"));
    }
}
