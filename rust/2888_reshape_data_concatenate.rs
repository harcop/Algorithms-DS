/// LeetCode #2888 - Reshape Data: Concatenate (Pandas; Rust analogue)
fn concatenate_tables(
    df1: Vec<(i32, String, i32)>,
    df2: Vec<(i32, String, i32)>,
) -> Vec<(i32, String, i32)> {
    let mut result = df1;
    result.extend(df2);
    result
}

fn main() {
    let df1 = vec![
        (1, "Mason".into(), 8),
        (2, "Ava".into(), 6),
    ];
    let df2 = vec![(5, "Leo".into(), 7)];
    println!("{:?}", concatenate_tables(df1, df2));
}

#[cfg(test)]
mod tests {
    use super::concatenate_tables;

    #[test]
    fn example() {
        let df1 = vec![
            (1, "Mason".into(), 8),
            (2, "Ava".into(), 6),
            (3, "Taylor".into(), 15),
            (4, "Georgia".into(), 17),
        ];
        let df2 = vec![(5, "Leo".into(), 7), (6, "Alex".into(), 7)];
        assert_eq!(
            concatenate_tables(df1, df2),
            vec![
                (1, "Mason".into(), 8),
                (2, "Ava".into(), 6),
                (3, "Taylor".into(), 15),
                (4, "Georgia".into(), 17),
                (5, "Leo".into(), 7),
                (6, "Alex".into(), 7),
            ]
        );
    }
}
