/// LeetCode #1072 - Flip Columns For Maximum Number of Equal Rows
fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let mut cnt: HashMap<Vec<i32>, i32> = HashMap::new();
    for row in matrix {
        let key: Vec<i32> = if row[0] == 0 {
            row
        } else {
            row.iter().map(|&x| 1 - x).collect()
        };
        *cnt.entry(key).or_default() += 1;
    }
    cnt.into_values().max().unwrap_or(0)
}

fn main() {
    println!("{}", max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 1]]));
}

#[cfg(test)]
mod tests {
    use super::max_equal_rows_after_flips;

    #[test]
    fn example_one() {
        assert_eq!(max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 1]]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 0]]), 2);
    }
}
