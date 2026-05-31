/// LeetCode #1568 - Minimum Number Of Days To Disconnect Island
fn min_days(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let dirs = [(0i32, 1), (0, -1), (1, 0), (-1, 0)];
    fn land(g: &[Vec<i32>], days: i32) -> Vec<Vec<bool>> {
        g.iter().map(|row| row.iter().map(|&v| v > days).collect()).collect()
    }
    fn dfs(g: &[Vec<bool>], vis: &mut [Vec<bool>], i: usize, j: usize, dirs: &[(i32, i32)]) {
        let n = g.len();
        let m = g[0].len();
        vis[i][j] = true;
        for (di, dj) in dirs {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && nj >= 0 && (ni as usize) < n && (nj as usize) < m {
                let ni = ni as usize;
                let nj = nj as usize;
                if g[ni][nj] && !vis[ni][nj] { dfs(g, vis, ni, nj, dirs); }
            }
        }
    }
    fn components(g: &[Vec<bool>], dirs: &[(i32, i32)]) -> i32 {
        let n = g.len();
        let m = g[0].len();
        let mut vis = vec![vec![false; m]; n];
        let mut c = 0;
        for i in 0..n {
            for j in 0..m {
                if g[i][j] && !vis[i][j] {
                    dfs(g, &mut vis, i, j, dirs);
                    c += 1;
                }
            }
        }
        c
    }
    fn can_disconnect(g: &[Vec<bool>], dirs: &[(i32, i32)]) -> bool {
        if components(g, dirs) != 1 { return false; }
        let n = g.len();
        let m = g[0].len();
        for i in 0..n {
            for j in 0..m {
                if !g[i][j] { continue; }
                let mut g2 = g.to_vec();
                g2[i][j] = false;
                if components(&g2, dirs) >= 2 { return true; }
            }
        }
        false
    }
    let mut hi = 0i32;
    for row in &grid {
        for &v in row {
            hi = hi.max(v);
        }
    }
    let mut lo = 0i32;
    let mut ans = hi + 1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if can_disconnect(&land(&grid, mid), &dirs) {
            ans = mid;
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    ans
}
fn main() { println!("{}", min_days(vec![vec![0,1,1,0],vec![1,1,1,0],vec![1,1,1,0],vec![0,1,1,0]])); }
#[cfg(test)]
mod tests {
    use super::min_days;
    #[test]
    fn example_one() { assert_eq!(min_days(vec![vec![0,1,1,0],vec![1,1,1,0],vec![1,1,1,0],vec![0,1,1,0]]), 2); }
    #[test]
    fn example_two() { assert_eq!(min_days(vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]]), 2); }
    #[test]
    fn example_three() { assert_eq!(min_days(vec![vec![1,1],vec![1,1]]), 2); }
}