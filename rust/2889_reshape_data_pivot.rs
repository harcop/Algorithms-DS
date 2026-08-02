/// LeetCode #2889 - Reshape Data: Pivot (Pandas; Rust analogue)
fn pivot_table(
    weather: Vec<(String, String, i32)>,
) -> Vec<(String, i32, i32)> {
    use std::collections::BTreeMap;

    let mut by_month: BTreeMap<String, (Option<i32>, Option<i32>)> = BTreeMap::new();
    for (city, month, temperature) in weather {
        let entry = by_month.entry(month).or_default();
        match city.as_str() {
            "ElPaso" => entry.0 = Some(temperature),
            "Jacksonville" => entry.1 = Some(temperature),
            _ => {}
        }
    }

    by_month
        .into_iter()
        .map(|(month, (el_paso, jacksonville))| {
            (month, el_paso.unwrap_or(0), jacksonville.unwrap_or(0))
        })
        .collect()
}

fn main() {
    let weather = vec![
        ("Jacksonville".into(), "January".into(), 13),
        ("ElPaso".into(), "January".into(), 20),
    ];
    println!("{:?}", pivot_table(weather));
}

#[cfg(test)]
mod tests {
    use super::pivot_table;

    #[test]
    fn example() {
        let weather = vec![
            ("Jacksonville".into(), "January".into(), 13),
            ("Jacksonville".into(), "February".into(), 23),
            ("Jacksonville".into(), "March".into(), 38),
            ("Jacksonville".into(), "April".into(), 5),
            ("Jacksonville".into(), "May".into(), 34),
            ("ElPaso".into(), "January".into(), 20),
            ("ElPaso".into(), "February".into(), 6),
            ("ElPaso".into(), "March".into(), 26),
            ("ElPaso".into(), "April".into(), 2),
            ("ElPaso".into(), "May".into(), 43),
        ];
        assert_eq!(
            pivot_table(weather),
            vec![
                ("April".into(), 2, 5),
                ("February".into(), 6, 23),
                ("January".into(), 20, 13),
                ("March".into(), 26, 38),
                ("May".into(), 43, 34),
            ]
        );
    }
}
