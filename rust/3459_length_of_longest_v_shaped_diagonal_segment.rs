/// LeetCode #3459 - Length of Longest V-Shaped Diagonal Segment
fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let dirs = [(1i32, 1i32), (1, -1), (-1, -1), (-1, 1)];
    let mut memo = vec![vec![vec![vec![-1i32; 2]; 4]; n]; m];
    fn dfs(
        i: i32,
        j: i32,
        k: usize,
        cnt: usize,
        grid: &[Vec<i32>],
        dirs: &[(i32, i32); 4],
        memo: &mut [Vec<Vec<Vec<i32>>>],
    ) -> i32 {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let (di, dj) = dirs[k];
        let x = i + di;
        let y = j + dj;
        let target = if grid[i as usize][j as usize] == 1 {
            2
        } else {
            2 - grid[i as usize][j as usize]
        };
        if x < 0 || x >= m || y < 0 || y >= n || grid[x as usize][y as usize] != target {
            return 0;
        }
        let xu = x as usize;
        let yu = y as usize;
        if memo[xu][yu][k][cnt] != -1 {
            return memo[xu][yu][k][cnt];
        }
        let mut res = dfs(x, y, k, cnt, grid, dirs, memo);
        if cnt > 0 {
            res = res.max(dfs(x, y, (k + 1) % 4, 0, grid, dirs, memo));
        }
        memo[xu][yu][k][cnt] = 1 + res;
        1 + res
    }
    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                for k in 0..4 {
                    ans = ans.max(dfs(i as i32, j as i32, k, 1, &grid, &dirs, &mut memo) + 1);
                }
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        len_of_v_diagonal(vec![
            vec![2, 2, 1, 2, 2],
            vec![2, 0, 2, 2, 0],
            vec![2, 0, 1, 1, 0],
            vec![1, 0, 2, 2, 2],
            vec![2, 0, 0, 2, 2],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::len_of_v_diagonal;

    #[test]
    fn example1() {
        assert_eq!(
            len_of_v_diagonal(vec![
                vec![2, 2, 1, 2, 2],
                vec![2, 0, 2, 2, 0],
                vec![2, 0, 1, 1, 0],
                vec![1, 0, 2, 2, 2],
                vec![2, 0, 0, 2, 2],
            ]),
            5
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            len_of_v_diagonal(vec![
                vec![2, 2, 2, 2, 2],
                vec![2, 0, 2, 2, 0],
                vec![2, 0, 1, 1, 0],
                vec![1, 0, 2, 2, 2],
                vec![2, 0, 0, 2, 2],
            ]),
            4
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            len_of_v_diagonal(vec![
                vec![1, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 0],
                vec![2, 0, 0, 0, 0],
                vec![0, 0, 2, 2, 2],
                vec![2, 0, 0, 2, 0],
            ]),
            5
        );
    }

    #[test]
    fn example4() {
        assert_eq!(len_of_v_diagonal(vec![vec![1]]), 1);
    }
}
