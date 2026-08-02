/// LeetCode #2890 - Reshape Data: Melt (Pandas; Rust analogue)
fn melt_table(
    report: Vec<(String, i32, i32, i32, i32)>,
) -> Vec<(String, String, i32)> {
    let quarters = ["quarter_1", "quarter_2", "quarter_3", "quarter_4"];
    let mut result = Vec::new();
    for quarter in quarters {
        for (product, q1, q2, q3, q4) in &report {
            let sales = match quarter {
                "quarter_1" => *q1,
                "quarter_2" => *q2,
                "quarter_3" => *q3,
                _ => *q4,
            };
            result.push((product.clone(), quarter.to_string(), sales));
        }
    }
    result
}

fn main() {
    let report = vec![
        ("Umbrella".into(), 417, 224, 379, 611),
        ("SleepingBag".into(), 800, 936, 93, 875),
    ];
    println!("{:?}", melt_table(report));
}

#[cfg(test)]
mod tests {
    use super::melt_table;

    #[test]
    fn example() {
        let report = vec![
            ("Umbrella".into(), 417, 224, 379, 611),
            ("SleepingBag".into(), 800, 936, 93, 875),
        ];
        assert_eq!(
            melt_table(report),
            vec![
                ("Umbrella".into(), "quarter_1".into(), 417),
                ("SleepingBag".into(), "quarter_1".into(), 800),
                ("Umbrella".into(), "quarter_2".into(), 224),
                ("SleepingBag".into(), "quarter_2".into(), 936),
                ("Umbrella".into(), "quarter_3".into(), 379),
                ("SleepingBag".into(), "quarter_3".into(), 93),
                ("Umbrella".into(), "quarter_4".into(), 611),
                ("SleepingBag".into(), "quarter_4".into(), 875),
            ]
        );
    }
}
