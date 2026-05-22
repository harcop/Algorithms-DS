/// LeetCode #1219 - Path with Maximum Gold
fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut best = 0i32;
    fn dfs(
        grid: &mut Vec<Vec<i32>>,
        r: usize,
        c: usize,
        sum: i32,
        best: &mut i32,
    ) {
        let g = grid[r][c];
        if g == 0 {
            return;
        }
        grid[r][c] = 0;
        let sum = sum + g;
        *best = (*best).max(sum);
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (dr, dc) in dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nc >= 0 && (nr as usize) < grid.len() && (nc as usize) < grid[0].len() {
                dfs(grid, nr as usize, nc as usize, sum, best);
            }
        }
        grid[r][c] = g;
    }
    let mut g = grid;
    for i in 0..n {
        for j in 0..m {
            if g[i][j] > 0 {
                dfs(&mut g, i, j, 0, &mut best);
            }
        }
    }
    best
}

fn main() {
    println!(
        "{}",
        get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::get_maximum_gold;

    #[test]
    fn example_one() {
        assert_eq!(
            get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]]),
            24
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            get_maximum_gold(vec![
                vec![1, 0, 7],
                vec![2, 0, 6],
                vec![3, 4, 5],
                vec![0, 3, 0],
                vec![9, 0, 20],
            ]),
            28
        );
    }
}
