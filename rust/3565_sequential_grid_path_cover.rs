/// LeetCode #3565 - Sequential Grid Path Cover
fn find_path(grid: Vec<Vec<i32>>, _k: i32) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let total = m * n;
    let dirs = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];
    let mut path = Vec::new();
    let mut st: u64 = 0;
    fn f(i: usize, j: usize, n: usize) -> usize {
        i * n + j
    }
    fn dfs(
        i: usize,
        j: usize,
        mut v: i32,
        grid: &[Vec<i32>],
        m: usize,
        n: usize,
        total: usize,
        dirs: &[(i32, i32)],
        path: &mut Vec<Vec<i32>>,
        st: &mut u64,
    ) -> bool {
        path.push(vec![i as i32, j as i32]);
        if path.len() == total {
            return true;
        }
        *st |= 1u64 << f(i, j, n);
        if grid[i][j] == v {
            v += 1;
        }
        for &(dx, dy) in dirs {
            let x = i as i32 + dx;
            let y = j as i32 + dy;
            if x >= 0 && y >= 0 {
                let x = x as usize;
                let y = y as usize;
                if x < m
                    && y < n
                    && (*st & (1u64 << f(x, y, n))) == 0
                    && (grid[x][y] == 0 || grid[x][y] == v)
                {
                    if dfs(x, y, v, grid, m, n, total, dirs, path, st) {
                        return true;
                    }
                }
            }
        }
        path.pop();
        *st ^= 1u64 << f(i, j, n);
        false
    }
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 0 || grid[i][j] == 1 {
                path.clear();
                st = 0;
                if dfs(i, j, 1, &grid, m, n, total, &dirs, &mut path, &mut st) {
                    return path;
                }
            }
        }
    }
    vec![]
}

fn main() {
    println!("{:?}", find_path(vec![vec![0, 0, 0], vec![0, 1, 2]], 2));
}

#[cfg(test)]
mod tests {
    use super::find_path;

    #[test]
    fn example1() {
        let path = find_path(vec![vec![0, 0, 0], vec![0, 1, 2]], 2);
        assert_eq!(path.len(), 6);
        let mut seen = std::collections::HashSet::new();
        for p in &path {
            assert!(seen.insert((p[0], p[1])));
        }
    }

    #[test]
    fn example2() {
        assert_eq!(find_path(vec![vec![1, 0, 4], vec![3, 0, 2]], 4), Vec::<Vec<i32>>::new());
    }
}
