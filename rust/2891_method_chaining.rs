/// LeetCode #2891 - Method Chaining (Pandas; Rust analogue)
fn find_heavy_animals(animals: Vec<(String, String, i32, i32)>) -> Vec<String> {
    let mut heavy: Vec<_> = animals
        .into_iter()
        .filter(|(_, _, _, weight)| *weight > 100)
        .collect();
    heavy.sort_by(|a, b| b.3.cmp(&a.3));
    heavy.into_iter().map(|(name, _, _, _)| name).collect()
}

fn main() {
    let animals = vec![
        ("Tatiana".into(), "Snake".into(), 98, 464),
        ("Khaled".into(), "Giraffe".into(), 50, 41),
        ("Alex".into(), "Leopard".into(), 6, 328),
    ];
    println!("{:?}", find_heavy_animals(animals));
}

#[cfg(test)]
mod tests {
    use super::find_heavy_animals;

    #[test]
    fn example() {
        let animals = vec![
            ("Tatiana".into(), "Snake".into(), 98, 464),
            ("Khaled".into(), "Giraffe".into(), 50, 41),
            ("Alex".into(), "Leopard".into(), 6, 328),
            ("Jonathan".into(), "Monkey".into(), 45, 463),
            ("Stefan".into(), "Bear".into(), 100, 50),
            ("Tommy".into(), "Panda".into(), 26, 349),
        ];
        assert_eq!(
            find_heavy_animals(animals),
            vec![
                "Tatiana".to_string(),
                "Jonathan".to_string(),
                "Tommy".to_string(),
                "Alex".to_string(),
            ]
        );
    }
}
