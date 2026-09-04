/// LeetCode #1841 - League Statistics (SQL; Rust analogue)
use std::collections::HashMap;

fn league_statistics(
    teams: Vec<(i32, String)>,
    matches: Vec<(i32, i32, i32, i32)>,
) -> Vec<(String, i32, i32, i32, i32, i32)> {
    let names: HashMap<i32, String> = teams.into_iter().collect();
    #[derive(Default)]
    struct Stat {
        played: i32,
        points: i32,
        gf: i32,
        ga: i32,
    }
    let mut stats: HashMap<i32, Stat> = HashMap::new();
    for (home, away, hg, ag) in matches {
        {
            let h = stats.entry(home).or_default();
            h.played += 1;
            h.gf += hg;
            h.ga += ag;
            h.points += if hg > ag {
                3
            } else if hg == ag {
                1
            } else {
                0
            };
        }
        {
            let a = stats.entry(away).or_default();
            a.played += 1;
            a.gf += ag;
            a.ga += hg;
            a.points += if ag > hg {
                3
            } else if ag == hg {
                1
            } else {
                0
            };
        }
    }
    let mut ans: Vec<(String, i32, i32, i32, i32, i32)> = stats
        .into_iter()
        .map(|(id, s)| {
            (
                names.get(&id).cloned().unwrap_or_default(),
                s.played,
                s.points,
                s.gf,
                s.ga,
                s.gf - s.ga,
            )
        })
        .collect();
    ans.sort_by(|a, b| {
        b.2.cmp(&a.2)
            .then(b.5.cmp(&a.5))
            .then(a.0.cmp(&b.0))
    });
    ans
}

fn main() {
    let teams = vec![
        (1, "Ajax".into()),
        (4, "Dortmund".into()),
        (6, "Arsenal".into()),
    ];
    let matches = vec![(1, 4, 0, 1), (1, 6, 3, 3), (4, 1, 5, 2), (6, 1, 0, 0)];
    println!("{:?}", league_statistics(teams, matches));
}

#[cfg(test)]
mod tests {
    use super::league_statistics;

    #[test]
    fn example_one() {
        let teams = vec![
            (1, "Ajax".into()),
            (4, "Dortmund".into()),
            (6, "Arsenal".into()),
        ];
        let matches = vec![(1, 4, 0, 1), (1, 6, 3, 3), (4, 1, 5, 2), (6, 1, 0, 0)];
        assert_eq!(
            league_statistics(teams, matches),
            vec![
                ("Dortmund".into(), 2, 6, 6, 2, 4),
                ("Arsenal".into(), 2, 2, 3, 3, 0),
                ("Ajax".into(), 4, 2, 5, 9, -4),
            ]
        );
    }
}
