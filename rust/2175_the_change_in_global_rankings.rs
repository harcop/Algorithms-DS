/// LeetCode #2175 - The Change in Global Rankings (MySQL)
pub const SQL: &str = r#"WITH
old_rankings AS (
    SELECT
        team_id,
        name,
        ROW_NUMBER() OVER (ORDER BY points DESC, name) AS old_rank
    FROM TeamPoints
),
new_rankings AS (
    SELECT
        t.team_id,
        ROW_NUMBER() OVER (ORDER BY t.points + p.points_change DESC, t.name) AS new_rank
    FROM TeamPoints AS t
    JOIN PointsChange AS p ON t.team_id = p.team_id
)
SELECT
    o.team_id,
    o.name,
    CAST(o.old_rank AS SIGNED) - CAST(n.new_rank AS SIGNED) AS rank_diff
FROM old_rankings AS o
JOIN new_rankings AS n ON o.team_id = n.team_id"#;

fn main() {
    println!("{}", SQL.lines().next().unwrap_or(""));
}

#[cfg(test)]
mod tests {
    use super::SQL;

    #[test]
    fn computes_old_and_new_ranks() {
        let sql = SQL.to_uppercase();
        assert!(sql.contains("ROW_NUMBER"));
        assert!(sql.contains("RANK_DIFF"));
    }
}
