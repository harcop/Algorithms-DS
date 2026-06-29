/// LeetCode #2173 - Longest Winning Streak (MySQL)
pub const SQL: &str = r#"WITH
matches_with_groups AS (
    SELECT
        player_id,
        match_day,
        result,
        SUM(result = 'Win') OVER (
            PARTITION BY player_id
            ORDER BY match_day
        ) - ROW_NUMBER() OVER (
            PARTITION BY player_id
            ORDER BY match_day
        ) AS grp
    FROM Matches
),
streaks AS (
    SELECT player_id, COUNT(*) AS streak
    FROM matches_with_groups
    WHERE result = 'Win'
    GROUP BY player_id, grp
)
SELECT
    players.player_id,
    COALESCE(MAX(s.streak), 0) AS longest_streak
FROM (SELECT DISTINCT player_id FROM Matches) AS players
LEFT JOIN streaks AS s ON players.player_id = s.player_id
GROUP BY players.player_id"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn uses_window_functions() {
        let sql = SQL.to_uppercase();
        assert!(sql.contains("ROW_NUMBER"));
        assert!(sql.contains("COALESCE"));
    }
}
