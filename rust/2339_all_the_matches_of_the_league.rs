/// LeetCode #2339 - All the Matches of the League (SQL; Rust analogue)
fn all_league_matches(teams: Vec<String>) -> Vec<(String, String)> {
    let mut ans = Vec::new();
    for home in &teams {
        for away in &teams {
            if home != away {
                ans.push((home.clone(), away.clone()));
            }
        }
    }
    ans.sort();
    ans
}

fn main() {
    println!(
        "{:?}",
        all_league_matches(vec![
            "Leetcode FC".into(),
            "Ahly SC".into(),
            "Real Madrid".into()
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::all_league_matches;

    #[test]
    fn example_one() {
        let teams = vec![
            "Leetcode FC".into(),
            "Ahly SC".into(),
            "Real Madrid".into(),
        ];
        let mut expected = vec![
            ("Real Madrid".into(), "Leetcode FC".into()),
            ("Real Madrid".into(), "Ahly SC".into()),
            ("Leetcode FC".into(), "Real Madrid".into()),
            ("Leetcode FC".into(), "Ahly SC".into()),
            ("Ahly SC".into(), "Real Madrid".into()),
            ("Ahly SC".into(), "Leetcode FC".into()),
        ];
        expected.sort();
        assert_eq!(all_league_matches(teams), expected);
    }
}
