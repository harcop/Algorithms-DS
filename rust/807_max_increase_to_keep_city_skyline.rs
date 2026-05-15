/// LeetCode #807 - Max Increase to Keep City Skyline
fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut row_max = vec![0i32; m];
    let mut col_max = vec![0i32; n];
    for i in 0..m {
        for j in 0..n {
            row_max[i] = row_max[i].max(grid[i][j]);
            col_max[j] = col_max[j].max(grid[i][j]);
        }
    }
    let mut ans = 0i32;
    for i in 0..m {
        for j in 0..n {
            ans += row_max[i].min(col_max[j]) - grid[i][j];
        }
    }
    ans
}

fn main() {
    let g = vec![vec![3, 0, 8, 4], vec![2, 4, 5, 7], vec![9, 2, 6, 3], vec![0, 3, 1, 0]];
    println!("{}", max_increase_keeping_skyline(g));
}

#[cfg(test)]
mod tests {
    use super::max_increase_keeping_skyline;

    #[test]
    fn example_one() {
        let g = vec![vec![3, 0, 8, 4], vec![2, 4, 5, 7], vec![9, 2, 6, 3], vec![0, 3, 1, 0]];
        assert_eq!(max_increase_keeping_skyline(g), 35);
    }
}
