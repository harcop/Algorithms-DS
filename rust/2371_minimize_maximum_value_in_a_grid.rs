/// LeetCode #2371 - Minimize Maximum Value in a Grid
fn min_score(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let mut ans = vec![vec![0; n]; m];
    let mut val_and_indices = Vec::new();
    let mut rows = vec![0; m];
    let mut cols = vec![0; n];

    for i in 0..m {
        for j in 0..n {
            val_and_indices.push((grid[i][j], i, j));
        }
    }
    val_and_indices.sort_unstable();

    for &(_, i, j) in &val_and_indices {
        let next = rows[i].max(cols[j]) + 1;
        ans[i][j] = next;
        rows[i] = next;
        cols[j] = next;
    }

    ans
}

fn main() {
    println!("{:?}", min_score(vec![vec![3, 1], vec![2, 5]]));
}

#[cfg(test)]
mod tests {
    use super::min_score;

    #[test]
    fn example_one() {
        assert_eq!(min_score(vec![vec![3, 1], vec![2, 5]]), vec![vec![2, 1], vec![1, 2]]);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_score(vec![vec![10]]), vec![vec![1]]);
    }
}
