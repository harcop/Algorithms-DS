/// LeetCode #1905 - Count Sub Islands
fn count_sub_islands(grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
    let m = grid1.len();
    let n = grid1[0].len();
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn dfs(
        i: i32,
        j: i32,
        grid1: &[Vec<i32>],
        grid2: &mut [Vec<i32>],
        m: i32,
        n: i32,
        dirs: &[(i32, i32)],
    ) -> bool {
        let (ui, uj) = (i as usize, j as usize);
        let mut ok = grid1[ui][uj] == 1;
        grid2[ui][uj] = 0;
        for &(a, b) in dirs {
            let x = i + a;
            let y = j + b;
            if 0 <= x && x < m && 0 <= y && y < n && grid2[x as usize][y as usize] == 1 {
                if !dfs(x, y, grid1, grid2, m, n, dirs) {
                    ok = false;
                }
            }
        }
        ok
    }

    let mut ans = 0i32;
    for i in 0..m {
        for j in 0..n {
            if grid2[i][j] == 1 && dfs(i as i32, j as i32, &grid1, &mut grid2, m as i32, n as i32, &dirs) {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    let grid1 = vec![
        vec![1, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0],
        vec![1, 1, 0, 1, 1],
    ];
    let grid2 = vec![
        vec![1, 1, 1, 0, 0],
        vec![0, 0, 1, 1, 1],
        vec![0, 1, 0, 0, 0],
        vec![1, 0, 1, 1, 0],
        vec![0, 1, 0, 1, 0],
    ];
    println!("{}", count_sub_islands(grid1, grid2));
}

#[cfg(test)]
mod tests {
    use super::count_sub_islands;

    #[test]
    fn example_one() {
        let grid1 = vec![
            vec![1, 1, 1, 0, 0],
            vec![0, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 1, 1],
        ];
        let grid2 = vec![
            vec![1, 1, 1, 0, 0],
            vec![0, 0, 1, 1, 1],
            vec![0, 1, 0, 0, 0],
            vec![1, 0, 1, 1, 0],
            vec![0, 1, 0, 1, 0],
        ];
        assert_eq!(count_sub_islands(grid1, grid2), 3);
    }
}
