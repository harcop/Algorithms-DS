/// LeetCode #1988 - Find Cutoff Score for Each School (MySQL)
pub const SQL: &str = r#"SELECT school_id, MIN(IFNULL(score, -1)) AS score
FROM Schools AS s
LEFT JOIN Exam AS e ON s.capacity >= e.student_count
GROUP BY school_id"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn uses_left_join() {
        assert!(SQL.to_uppercase().contains("LEFT JOIN"));
    }
}
