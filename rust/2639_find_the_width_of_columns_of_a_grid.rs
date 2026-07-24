/// LeetCode #2639 - Find the Width of Columns of a Grid
fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid[0].len();
    let mut ans = vec![0; n];
    for row in &grid {
        for (j, &x) in row.iter().enumerate() {
            let w = x.to_string().len() as i32;
            ans[j] = ans[j].max(w);
        }
    }
    ans
}

fn main() {
    println!("{:?}", find_column_width(vec![vec![1], vec![22], vec![333]]));
}

#[cfg(test)]
mod tests {
    use super::find_column_width;

    #[test]
    fn example_one() {
        assert_eq!(
            find_column_width(vec![vec![1], vec![22], vec![333]]),
            vec![3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_column_width(vec![vec![-15, 1, 3], vec![15, 7, 12], vec![5, 6, -2]]),
            vec![3, 1, 2]
        );
    }
}
