/// LeetCode #2617 - Minimum Number of Visited Cells in a Grid
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn minimum_visited_cells(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dist = vec![vec![-1; n]; m];
    dist[0][0] = 1;
    let mut row: Vec<BinaryHeap<Reverse<(i32, usize)>>> = (0..m).map(|_| BinaryHeap::new()).collect();
    let mut col: Vec<BinaryHeap<Reverse<(i32, usize)>>> = (0..n).map(|_| BinaryHeap::new()).collect();

    for i in 0..m {
        for j in 0..n {
            while let Some(Reverse((_, jj))) = row[i].peek().copied() {
                if grid[i][jj] as usize + jj < j {
                    row[i].pop();
                } else {
                    break;
                }
            }
            if let Some(Reverse((d, _))) = row[i].peek().copied() {
                if dist[i][j] == -1 || d + 1 < dist[i][j] {
                    dist[i][j] = d + 1;
                }
            }

            while let Some(Reverse((_, ii))) = col[j].peek().copied() {
                if grid[ii][j] as usize + ii < i {
                    col[j].pop();
                } else {
                    break;
                }
            }
            if let Some(Reverse((d, _))) = col[j].peek().copied() {
                if dist[i][j] == -1 || d + 1 < dist[i][j] {
                    dist[i][j] = d + 1;
                }
            }

            if dist[i][j] != -1 {
                row[i].push(Reverse((dist[i][j], j)));
                col[j].push(Reverse((dist[i][j], i)));
            }
        }
    }
    dist[m - 1][n - 1]
}

fn main() {
    println!(
        "{}",
        minimum_visited_cells(vec![
            vec![3, 4, 2, 1],
            vec![4, 2, 3, 1],
            vec![2, 1, 0, 0],
            vec![2, 4, 0, 0]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_visited_cells;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_visited_cells(vec![
                vec![3, 4, 2, 1],
                vec![4, 2, 3, 1],
                vec![2, 1, 0, 0],
                vec![2, 4, 0, 0]
            ]),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_visited_cells(vec![
                vec![3, 4, 2, 1],
                vec![4, 2, 1, 1],
                vec![2, 1, 1, 0],
                vec![3, 4, 1, 0]
            ]),
            3
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            minimum_visited_cells(vec![vec![2, 1, 0], vec![1, 0, 0]]),
            -1
        );
    }
}
