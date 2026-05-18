/// LeetCode #934 - Shortest Bridge
use std::collections::VecDeque;

fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut q = VecDeque::new();

    fn dfs(g: &mut Vec<Vec<i32>>, r: usize, c: usize, q: &mut VecDeque<(usize, usize)>) {
        g[r][c] = 2;
        q.push_back((r, c));
        if r > 0 && g[r - 1][c] == 1 {
            dfs(g, r - 1, c, q);
        }
        if r + 1 < g.len() && g[r + 1][c] == 1 {
            dfs(g, r + 1, c, q);
        }
        if c > 0 && g[r][c - 1] == 1 {
            dfs(g, r, c - 1, q);
        }
        if c + 1 < g[0].len() && g[r][c + 1] == 1 {
            dfs(g, r, c + 1, q);
        }
    }

    'outer: for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 {
                dfs(&mut grid, i, j, &mut q);
                break 'outer;
            }
        }
    }

    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut ans = 0;
    loop {
        for _ in 0..q.len() {
            let (r, c) = q.pop_front().unwrap();
            for (dr, dc) in dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nc < 0 || nr >= n as i32 || nc >= n as i32 {
                    continue;
                }
                let nr = nr as usize;
                let nc = nc as usize;
                if grid[nr][nc] == 1 {
                    return ans;
                }
                if grid[nr][nc] == 0 {
                    grid[nr][nc] = 2;
                    q.push_back((nr, nc));
                }
            }
        }
        ans += 1;
    }
}

fn main() {
    let g = vec![vec![0, 1], vec![1, 0]];
    println!("{}", shortest_bridge(g));
}

#[cfg(test)]
mod tests {
    use super::shortest_bridge;

    #[test]
    fn example_one() {
        let g = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(shortest_bridge(g), 1);
    }

    #[test]
    fn example_two() {
        let g = vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]];
        assert_eq!(shortest_bridge(g), 2);
    }

    #[test]
    fn separated_islands() {
        let g = vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 1],
            vec![0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(shortest_bridge(g), 3);
    }
}
