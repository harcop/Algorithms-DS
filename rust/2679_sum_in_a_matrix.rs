/// LeetCode #2679 - Sum in a Matrix
fn matrix_sum(mut nums: Vec<Vec<i32>>) -> i32 {
    for row in &mut nums {
        row.sort_unstable();
    }
    (0..nums[0].len())
        .map(|col| nums.iter().map(|row| row[col]).max().unwrap())
        .sum()
}

fn main() {
    println!(
        "{}",
        matrix_sum(vec![
            vec![7, 2, 1],
            vec![6, 4, 2],
            vec![6, 5, 3],
            vec![3, 2, 1]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::matrix_sum;

    #[test]
    fn example_one() {
        assert_eq!(
            matrix_sum(vec![
                vec![7, 2, 1],
                vec![6, 4, 2],
                vec![6, 5, 3],
                vec![3, 2, 1]
            ]),
            15
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(matrix_sum(vec![vec![1]]), 1);
    }
}
