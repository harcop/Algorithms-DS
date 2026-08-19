/// LeetCode #3311 - Construct 2D Grid Matching Graph Layout
fn construct_grid_layout(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut g = vec![Vec::new(); n];
    for e in &edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }
    let mut deg = [-1i32; 5];
    for (x, ys) in g.iter().enumerate() {
        deg[ys.len()] = x as i32;
    }
    let mut row: Vec<i32> = if deg[1] != -1 {
        vec![deg[1]]
    } else if deg[4] == -1 {
        let x = deg[2] as usize;
        let mut found = vec![];
        for &y in &g[x] {
            if g[y].len() == 2 {
                found = vec![x as i32, y as i32];
                break;
            }
        }
        found
    } else {
        let mut x = deg[2] as usize;
        let mut row = vec![x as i32];
        let mut pre = x;
        x = g[x][0];
        while g[x].len() > 2 {
            row.push(x as i32);
            for &y in &g[x] {
                if y != pre && g[y].len() < 4 {
                    pre = x;
                    x = y;
                    break;
                }
            }
        }
        row.push(x as i32);
        row
    };
    let mut ans = vec![row.clone()];
    let mut vis = vec![false; n];
    for _ in 0..n / row.len() - 1 {
        for &x in &row {
            vis[x as usize] = true;
        }
        let mut nxt = Vec::new();
        for &x in &row {
            for &y in &g[x as usize] {
                if !vis[y] {
                    nxt.push(y as i32);
                    break;
                }
            }
        }
        ans.push(nxt.clone());
        row = nxt;
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        construct_grid_layout(4, vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::construct_grid_layout;
    use std::collections::HashSet;

    fn valid(n: i32, edges: &[Vec<i32>], grid: &[Vec<i32>]) {
        let n = n as usize;
        let mut pos = vec![None; n];
        let mut seen = HashSet::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                assert!(seen.insert(v));
                pos[v as usize] = Some((i as i32, j as i32));
            }
        }
        assert_eq!(seen.len(), n);
        let mut edge_set = HashSet::new();
        for e in edges {
            edge_set.insert((e[0].min(e[1]), e[0].max(e[1])));
        }
        let dirs = [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)];
        let mut grid_edges = HashSet::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                for (di, dj) in dirs {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && ni < grid.len() as i32 && nj >= 0 && nj < row.len() as i32 {
                        let u = grid[ni as usize][nj as usize];
                        grid_edges.insert((v.min(u), v.max(u)));
                    }
                }
            }
        }
        assert_eq!(grid_edges, edge_set);
    }

    #[test]
    fn example1() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]];
        let grid = construct_grid_layout(4, edges.clone());
        valid(4, &edges, &grid);
    }

    #[test]
    fn example2() {
        let edges = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![2, 4]];
        let grid = construct_grid_layout(5, edges.clone());
        valid(5, &edges, &grid);
    }

    #[test]
    fn example3() {
        let edges = vec![
            vec![0, 1],
            vec![0, 4],
            vec![0, 5],
            vec![1, 7],
            vec![2, 3],
            vec![2, 4],
            vec![2, 5],
            vec![3, 6],
            vec![4, 6],
            vec![4, 7],
            vec![6, 8],
            vec![7, 8],
        ];
        let grid = construct_grid_layout(9, edges.clone());
        valid(9, &edges, &grid);
    }
}
