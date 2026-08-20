/// LeetCode #3322 - Premier League Table Ranking III (SQL; Rust analogue)
/// row: (season_id, team_id, team_name, matches_played, wins, draws, losses, goals_for, goals_against)
fn premier_league_table_ranking_iii(
    season_stats: Vec<(i32, i32, String, i32, i32, i32, i32, i32, i32)>,
) -> Vec<(i32, i32, String, i32, i32, i32)> {
    let mut rows: Vec<(i32, i32, String, i32, i32)> = season_stats
        .into_iter()
        .map(|(sid, tid, name, _, wins, draws, _, gf, ga)| {
            (sid, tid, name, wins * 3 + draws, gf - ga)
        })
        .collect();
    rows.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then(b.3.cmp(&a.3))
            .then(b.4.cmp(&a.4))
            .then(a.2.cmp(&b.2))
    });
    let mut ans = Vec::with_capacity(rows.len());
    let mut i = 0;
    while i < rows.len() {
        let sid = rows[i].0;
        let mut j = i;
        while j < rows.len() && rows[j].0 == sid {
            j += 1;
        }
        for (pos, idx) in (i..j).enumerate() {
            let (sid, tid, name, points, gd) = rows[idx].clone();
            ans.push((sid, tid, name, points, gd, (pos + 1) as i32));
        }
        i = j;
    }
    ans.sort_by(|a, b| a.0.cmp(&b.0).then(a.5.cmp(&b.5)).then(a.2.cmp(&b.2)));
    ans
}

fn main() {
    let stats = vec![(
        2021,
        1,
        "Manchester City".into(),
        38,
        29,
        6,
        3,
        99,
        26,
    )];
    println!("{:?}", premier_league_table_ranking_iii(stats));
}

#[cfg(test)]
mod tests {
    use super::premier_league_table_ranking_iii;

    #[test]
    fn example() {
        let stats = vec![
            (2021, 1, "Manchester City".into(), 38, 29, 6, 3, 99, 26),
            (2021, 2, "Liverpool".into(), 38, 28, 8, 2, 94, 26),
            (2021, 3, "Chelsea".into(), 38, 21, 11, 6, 76, 33),
            (2021, 4, "Tottenham".into(), 38, 22, 5, 11, 69, 40),
            (2021, 5, "Arsenal".into(), 38, 22, 3, 13, 61, 48),
            (2022, 1, "Manchester City".into(), 38, 28, 5, 5, 94, 33),
            (2022, 2, "Arsenal".into(), 38, 26, 6, 6, 88, 43),
            (2022, 3, "Manchester United".into(), 38, 23, 6, 9, 58, 43),
            (2022, 4, "Newcastle".into(), 38, 19, 14, 5, 68, 33),
            (2022, 5, "Liverpool".into(), 38, 19, 10, 9, 75, 47),
        ];
        assert_eq!(
            premier_league_table_ranking_iii(stats),
            vec![
                (2021, 1, "Manchester City".into(), 93, 73, 1),
                (2021, 2, "Liverpool".into(), 92, 68, 2),
                (2021, 3, "Chelsea".into(), 74, 43, 3),
                (2021, 4, "Tottenham".into(), 71, 29, 4),
                (2021, 5, "Arsenal".into(), 69, 13, 5),
                (2022, 1, "Manchester City".into(), 89, 61, 1),
                (2022, 2, "Arsenal".into(), 84, 45, 2),
                (2022, 3, "Manchester United".into(), 75, 15, 3),
                (2022, 4, "Newcastle".into(), 71, 35, 4),
                (2022, 5, "Liverpool".into(), 67, 28, 5),
            ]
        );
    }
}
