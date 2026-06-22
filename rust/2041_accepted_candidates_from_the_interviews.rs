/// LeetCode #2041 - Accepted Candidates From the Interviews (MySQL)
pub const SQL: &str = r#"SELECT candidate_id
FROM Candidates
JOIN Rounds USING (interview_id)
WHERE years_of_exp >= 2
GROUP BY 1
HAVING SUM(score) > 15"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn uses_having() {
        assert!(SQL.to_uppercase().contains("HAVING"));
    }
}
