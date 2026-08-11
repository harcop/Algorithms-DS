/// LeetCode #3148 - Maximum Difference Score in a Grid
fn max_score(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut f = vec![vec![0i32; n]; m];
    let mut ans = i32::MIN;
    for i in 0..m {
        for j in 0..n {
            let x = grid[i][j];
            let mut mi = i32::MAX;
            if i > 0 {
                mi = mi.min(f[i - 1][j]);
            }
            if j > 0 {
                mi = mi.min(f[i][j - 1]);
            }
            ans = ans.max(x - mi);
            f[i][j] = x.min(mi);
        }
    }
    ans
}

fn main() {
    let grid = vec![
        vec![9, 5, 7, 3],
        vec![8, 9, 6, 1],
        vec![6, 7, 14, 3],
        vec![2, 5, 3, 1],
    ];
    println!("{}", max_score(grid));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example1() {
        let grid = vec![
            vec![9, 5, 7, 3],
            vec![8, 9, 6, 1],
            vec![6, 7, 14, 3],
            vec![2, 5, 3, 1],
        ];
        assert_eq!(max_score(grid), 9);
    }

    #[test]
    fn example2() {
        let grid = vec![vec![4, 3, 2], vec![3, 2, 1]];
        assert_eq!(max_score(grid), -1);
    }
}
