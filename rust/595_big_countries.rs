/// LeetCode #595 - Big Countries (SQL; Rust analogue)
fn big_countries(world: Vec<(String, String, i64, i64, i64)>) -> Vec<(String, i64, i64)> {
    world
        .into_iter()
        .filter(|(_, _, area, pop, _)| *area >= 3_000_000 || *pop >= 25_000_000)
        .map(|(name, _, area, pop, _)| (name, pop, area))
        .collect()
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::big_countries;

    #[test]
    fn example() {
        let world = vec![
            ("Afghanistan".into(), "Asia".into(), 652230, 25500100, 20343000000),
            ("Albania".into(), "Europe".into(), 28748, 2831741, 12960000000),
            ("Algeria".into(), "Africa".into(), 2381741, 37100000, 188681000000),
            ("Andorra".into(), "Europe".into(), 468, 78115, 3712000000),
            ("Angola".into(), "Africa".into(), 1246700, 20609294, 100990000000),
        ];
        assert_eq!(
            big_countries(world),
            vec![
                ("Afghanistan".into(), 25500100, 652230),
                ("Algeria".into(), 37100000, 2381741),
            ]
        );
    }
}
