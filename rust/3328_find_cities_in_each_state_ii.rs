/// LeetCode #3328 - Find Cities in Each State II (SQL; Rust analogue)
use std::collections::HashMap;

fn state_city_analysis(cities: Vec<(String, String)>) -> Vec<(String, String, i32)> {
    let mut by_state: HashMap<String, Vec<String>> = HashMap::new();
    for (state, city) in cities {
        by_state.entry(state).or_default().push(city);
    }
    let mut ans = Vec::new();
    for (state, mut cs) in by_state {
        if cs.len() < 3 {
            continue;
        }
        let first = state.chars().next().unwrap();
        let matching = cs
            .iter()
            .filter(|c| c.chars().next() == Some(first))
            .count() as i32;
        if matching == 0 {
            continue;
        }
        cs.sort_by_key(|c| c.replace(' ', "").to_lowercase());
        ans.push((state, cs.join(", "), matching));
    }
    ans.sort_by(|a, b| b.2.cmp(&a.2).then(a.0.cmp(&b.0)));
    ans
}

fn main() {
    let cities = vec![
        ("New York".into(), "Newark".into()),
        ("New York".into(), "Buffalo".into()),
        ("New York".into(), "Rochester".into()),
    ];
    println!("{:?}", state_city_analysis(cities));
}

#[cfg(test)]
mod tests {
    use super::state_city_analysis;

    #[test]
    fn example() {
        let cities = vec![
            ("New York".into(), "New York City".into()),
            ("New York".into(), "Newark".into()),
            ("New York".into(), "Buffalo".into()),
            ("New York".into(), "Rochester".into()),
            ("California".into(), "San Francisco".into()),
            ("California".into(), "Sacramento".into()),
            ("California".into(), "San Diego".into()),
            ("California".into(), "Los Angeles".into()),
            ("Texas".into(), "Tyler".into()),
            ("Texas".into(), "Temple".into()),
            ("Texas".into(), "Taylor".into()),
            ("Texas".into(), "Dallas".into()),
            ("Pennsylvania".into(), "Philadelphia".into()),
            ("Pennsylvania".into(), "Pittsburgh".into()),
            ("Pennsylvania".into(), "Pottstown".into()),
        ];
        assert_eq!(
            state_city_analysis(cities),
            vec![
                (
                    "Pennsylvania".into(),
                    "Philadelphia, Pittsburgh, Pottstown".into(),
                    3
                ),
                (
                    "Texas".into(),
                    "Dallas, Taylor, Temple, Tyler".into(),
                    3
                ),
                (
                    "New York".into(),
                    "Buffalo, Newark, New York City, Rochester".into(),
                    2
                ),
            ]
        );
    }
}
