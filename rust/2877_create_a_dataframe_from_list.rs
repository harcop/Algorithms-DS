/// LeetCode #2877 - Create a DataFrame from List (Pandas; Rust analogue)
fn create_dataframe(student_data: Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    student_data
        .into_iter()
        .map(|row| (row[0], row[1]))
        .collect()
}

fn main() {
    println!(
        "{:?}",
        create_dataframe(vec![vec![1, 15], vec![2, 11], vec![3, 11], vec![4, 20]])
    );
}

#[cfg(test)]
mod tests {
    use super::create_dataframe;

    #[test]
    fn example() {
        assert_eq!(
            create_dataframe(vec![vec![1, 15], vec![2, 11], vec![3, 11], vec![4, 20]]),
            vec![(1, 15), (2, 11), (3, 11), (4, 20)]
        );
    }
}
