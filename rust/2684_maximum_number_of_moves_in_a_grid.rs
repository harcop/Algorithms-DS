/// LeetCode #2684 - Maximum Number of Moves in a Grid
use std::collections::HashSet;

fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut q: HashSet<usize> = (0..m).collect();
    for j in 0..n - 1 {
        let mut t = HashSet::new();
        for &i in &q {
            for k in i.saturating_sub(1)..=(i + 1).min(m - 1) {
                if grid[i][j] < grid[k][j + 1] {
                    t.insert(k);
                }
            }
        }
        if t.is_empty() {
            return j as i32;
        }
        q = t;
    }
    (n - 1) as i32
}

fn main() {
    println!(
        "{}",
        max_moves(vec![
            vec![2, 4, 3, 5],
            vec![5, 4, 9, 3],
            vec![3, 4, 2, 11],
            vec![10, 9, 13, 15],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::max_moves;

    #[test]
    fn example_one() {
        assert_eq!(
            max_moves(vec![
                vec![2, 4, 3, 5],
                vec![5, 4, 9, 3],
                vec![3, 4, 2, 11],
                vec![10, 9, 13, 15],
            ]),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_moves(vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]]),
            0
        );
    }
}
