/// LeetCode #3071 - Minimum Operations to Write the Letter Y on a Grid
use std::collections::HashSet;

fn minimum_operations_to_write_y(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let center = n / 2;
    let mut y_cells = HashSet::new();

    for i in 0..=center {
        y_cells.insert((i, i));
        y_cells.insert((i, n - 1 - i));
    }
    for i in center..n {
        y_cells.insert((i, center));
    }

    let mut freq_y = [0i32; 3];
    let mut freq_non = [0i32; 3];

    for i in 0..n {
        for j in 0..n {
            let v = grid[i][j] as usize;
            if y_cells.contains(&(i, j)) {
                freq_y[v] += 1;
            } else {
                freq_non[v] += 1;
            }
        }
    }

    let total = (n * n) as i32;
    let mut best = i32::MAX;
    for i in 0..3 {
        for j in 0..3 {
            if i != j {
                best = best.min(total - freq_y[i] - freq_non[j]);
            }
        }
    }

    best
}

fn main() {
    let grid = vec![vec![1, 2, 2], vec![1, 1, 0], vec![0, 1, 0]];
    println!("{}", minimum_operations_to_write_y(grid));
}

#[cfg(test)]
mod tests {
    use super::minimum_operations_to_write_y;

    #[test]
    fn example1() {
        let grid = vec![vec![1, 2, 2], vec![1, 1, 0], vec![0, 1, 0]];
        assert_eq!(minimum_operations_to_write_y(grid), 3);
    }

    #[test]
    fn example2() {
        let grid = vec![
            vec![0, 1, 0, 1, 0],
            vec![2, 1, 0, 1, 2],
            vec![2, 2, 2, 0, 1],
            vec![2, 2, 2, 2, 2],
            vec![2, 1, 2, 2, 2],
        ];
        assert_eq!(minimum_operations_to_write_y(grid), 12);
    }
}
