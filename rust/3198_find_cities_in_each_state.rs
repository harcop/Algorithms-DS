/// LeetCode #3198 - Find Cities in Each State (SQL; Rust analogue)
use std::collections::BTreeMap;

/// cities: (state, city)
fn find_cities(cities: Vec<(String, String)>) -> Vec<(String, String)> {
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (state, city) in cities {
        map.entry(state).or_default().push(city);
    }
    map.into_iter()
        .map(|(state, mut cs)| {
            cs.sort();
            (state, cs.join(", "))
        })
        .collect()
}

fn main() {
    let cities = vec![
        ("California".into(), "Los Angeles".into()),
        ("California".into(), "San Francisco".into()),
        ("Texas".into(), "Houston".into()),
    ];
    println!("{:?}", find_cities(cities));
}

#[cfg(test)]
mod tests {
    use super::find_cities;

    #[test]
    fn example() {
        let cities = vec![
            ("California".into(), "Los Angeles".into()),
            ("California".into(), "San Francisco".into()),
            ("California".into(), "San Diego".into()),
            ("Texas".into(), "Houston".into()),
            ("Texas".into(), "Austin".into()),
            ("Texas".into(), "Dallas".into()),
            ("New York".into(), "New York City".into()),
            ("New York".into(), "Buffalo".into()),
            ("New York".into(), "Rochester".into()),
        ];
        assert_eq!(
            find_cities(cities),
            vec![
                (
                    "California".into(),
                    "Los Angeles, San Diego, San Francisco".into()
                ),
                (
                    "New York".into(),
                    "Buffalo, New York City, Rochester".into()
                ),
                ("Texas".into(), "Austin, Dallas, Houston".into()),
            ]
        );
    }
}
