/// LeetCode #3070 - Count Submatrices with Top-Left Element and Sum Less Than k
fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut prefix = vec![vec![0i64; n + 1]; m + 1];

    for i in 0..m {
        for j in 0..n {
            prefix[i + 1][j + 1] = prefix[i + 1][j]
                + prefix[i][j + 1]
                - prefix[i][j]
                + grid[i][j] as i64;
        }
    }

    let mut count = 0;
    for i in 0..m {
        for j in 0..n {
            if prefix[i + 1][j + 1] <= k as i64 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let grid = vec![vec![7, 6, 3], vec![6, 6, 1]];
    println!("{}", count_submatrices(grid, 18));
}

#[cfg(test)]
mod tests {
    use super::count_submatrices;

    #[test]
    fn example1() {
        let grid = vec![vec![7, 6, 3], vec![6, 6, 1]];
        assert_eq!(count_submatrices(grid, 18), 4);
    }

    #[test]
    fn example2() {
        let grid = vec![
            vec![7, 2, 9],
            vec![1, 5, 0],
            vec![2, 6, 6],
        ];
        assert_eq!(count_submatrices(grid, 20), 6);
    }
}
