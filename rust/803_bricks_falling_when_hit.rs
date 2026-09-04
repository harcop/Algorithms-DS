/// LeetCode #803 - Bricks Falling When Hit
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let pa = self.find(a);
        let pb = self.find(b);
        if pa == pb {
            return;
        }
        self.parent[pa] = pb;
        self.size[pb] += self.size[pa];
    }

    fn sz(&mut self, x: usize) -> usize {
        let p = self.find(x);
        self.size[p]
    }
}

fn hit_bricks(mut grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
    let m = grid.len();
    let n = grid[0].len();
    let dirs = [(0isize, 1isize), (0, -1), (1, 0), (-1, 0)];
    let id = |r: usize, c: usize| r * n + c;
    let roof = m * n;

    for h in &hits {
        let r = h[0] as usize;
        let c = h[1] as usize;
        if grid[r][c] == 1 {
            grid[r][c] = 2;
        } else {
            grid[r][c] = -1;
        }
    }

    let mut uf = UnionFind::new(m * n + 1);
    for r in 0..m {
        for c in 0..n {
            if grid[r][c] != 1 {
                continue;
            }
            if r == 0 {
                uf.union(id(r, c), roof);
            }
            for (dr, dc) in dirs {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr >= 0 && nc >= 0 && (nr as usize) < m && (nc as usize) < n {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if grid[nr][nc] == 1 {
                        uf.union(id(r, c), id(nr, nc));
                    }
                }
            }
        }
    }

    let mut ans = vec![0; hits.len()];
    for i in (0..hits.len()).rev() {
        let r = hits[i][0] as usize;
        let c = hits[i][1] as usize;
        if grid[r][c] == -1 {
            grid[r][c] = 0;
            ans[i] = 0;
            continue;
        }
        let before = uf.sz(roof);
        grid[r][c] = 1;
        if r == 0 {
            uf.union(id(r, c), roof);
        }
        for (dr, dc) in dirs {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nc >= 0 && (nr as usize) < m && (nc as usize) < n {
                let nr = nr as usize;
                let nc = nc as usize;
                if grid[nr][nc] == 1 {
                    uf.union(id(r, c), id(nr, nc));
                }
            }
        }
        let after = uf.sz(roof);
        ans[i] = (after.saturating_sub(before + 1)) as i32;
    }
    ans
}

fn main() {
    let grid = vec![vec![1, 0, 0, 0], vec![1, 1, 1, 0]];
    let hits = vec![vec![1, 0]];
    println!("{:?}", hit_bricks(grid, hits));
}

#[cfg(test)]
mod tests {
    use super::hit_bricks;

    #[test]
    fn example_one() {
        let grid = vec![vec![1, 0, 0, 0], vec![1, 1, 1, 0]];
        let hits = vec![vec![1, 0]];
        assert_eq!(hit_bricks(grid, hits), vec![2]);
    }

    #[test]
    fn example_two() {
        let grid = vec![vec![1, 0, 0, 0], vec![1, 1, 0, 0]];
        let hits = vec![vec![1, 1], vec![1, 0]];
        assert_eq!(hit_bricks(grid, hits), vec![0, 0]);
    }
}
