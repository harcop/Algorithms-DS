/// LeetCode #2304 - Minimum Path Cost in a Grid
fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut f = grid[0].clone();
    for i in 1..m {
        let mut g = vec![i32::MAX; n];
        for j in 0..n {
            for k in 0..n {
                g[j] = g[j].min(f[k] + move_cost[grid[i - 1][k] as usize][j] + grid[i][j]);
            }
        }
        f = g;
    }
    *f.iter().min().unwrap()
}

fn main() {
    println!(
        "{}",
        min_path_cost(
            vec![vec![5, 3], vec![4, 0], vec![2, 1]],
            vec![
                vec![9, 8],
                vec![1, 5],
                vec![10, 12],
                vec![18, 6],
                vec![2, 4],
                vec![14, 3]
            ]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::min_path_cost;

    #[test]
    fn example_one() {
        assert_eq!(
            min_path_cost(
                vec![vec![5, 3], vec![4, 0], vec![2, 1]],
                vec![
                    vec![9, 8],
                    vec![1, 5],
                    vec![10, 12],
                    vec![18, 6],
                    vec![2, 4],
                    vec![14, 3]
                ]
            ),
            17
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            min_path_cost(
                vec![vec![5, 1, 2], vec![4, 0, 3]],
                vec![
                    vec![12, 10, 15],
                    vec![20, 23, 8],
                    vec![21, 7, 1],
                    vec![8, 1, 13],
                    vec![9, 10, 25],
                    vec![5, 3, 2]
                ]
            ),
            6
        );
    }
}
