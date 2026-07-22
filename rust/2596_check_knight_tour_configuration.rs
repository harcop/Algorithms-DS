/// LeetCode #2596 - Check Knight Tour Configuration
fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
    if grid[0][0] != 0 {
        return false;
    }
    let n = grid.len();
    let mut pos = vec![(0usize, 0usize); n * n];
    for i in 0..n {
        for j in 0..n {
            pos[grid[i][j] as usize] = (i, j);
        }
    }
    for i in 1..n * n {
        let (x1, y1) = pos[i - 1];
        let (x2, y2) = pos[i];
        let dx = (x1 as i32 - x2 as i32).abs();
        let dy = (y1 as i32 - y2 as i32).abs();
        let ok = (dx == 1 && dy == 2) || (dx == 2 && dy == 1);
        if !ok {
            return false;
        }
    }
    true
}

fn main() {
    println!(
        "{}",
        check_valid_grid(vec![
            vec![0, 11, 16, 5, 20],
            vec![17, 4, 19, 10, 15],
            vec![12, 1, 8, 21, 6],
            vec![3, 18, 23, 14, 9],
            vec![24, 13, 2, 7, 22]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::check_valid_grid;

    #[test]
    fn example_one() {
        assert!(check_valid_grid(vec![
            vec![0, 11, 16, 5, 20],
            vec![17, 4, 19, 10, 15],
            vec![12, 1, 8, 21, 6],
            vec![3, 18, 23, 14, 9],
            vec![24, 13, 2, 7, 22]
        ]));
    }

    #[test]
    fn example_two() {
        assert!(!check_valid_grid(vec![
            vec![0, 3, 6],
            vec![5, 8, 1],
            vec![2, 7, 4]
        ]));
    }
}
