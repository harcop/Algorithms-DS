/// LeetCode #827 - Making a Large Island
fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    if n == 0 {
        return 0;
    }
    let mut grid = grid;
    let mut id = 2;
    let mut areas = vec![0i32; n * n + 2];

    fn dfs(grid: &mut [Vec<i32>], r: usize, c: usize, id: i32) -> i32 {
        let n = grid.len();
        if r >= n || c >= n || grid[r][c] != 1 {
            return 0;
        }
        grid[r][c] = id;
        1 + dfs(grid, r + 1, c, id)
            + dfs(grid, r.wrapping_sub(1), c, id)
            + dfs(grid, r, c + 1, id)
            + dfs(grid, r, c.wrapping_sub(1), id)
    }

    for r in 0..n {
        for c in 0..n {
            if grid[r][c] == 1 {
                areas[id as usize] = dfs(&mut grid, r, c, id);
                id += 1;
            }
        }
    }

    let mut ans = areas[2..id as usize].iter().copied().max().unwrap_or(0);
    for r in 0..n {
        for c in 0..n {
            if grid[r][c] != 0 {
                continue;
            }
            let mut seen = std::collections::HashSet::new();
            let mut size = 1;
            for (nr, nc) in [(r + 1, c), (r.wrapping_sub(1), c), (r, c + 1), (r, c.wrapping_sub(1))]
            {
                if nr < n && nc < n && grid[nr][nc] > 1 && seen.insert(grid[nr][nc]) {
                    size += areas[grid[nr][nc] as usize];
                }
            }
            ans = ans.max(size);
        }
    }
    ans
}

fn main() {
    println!("{}", largest_island(vec![vec![1, 0], vec![0, 1]]));
}

#[cfg(test)]
mod tests {
    use super::largest_island;

    #[test]
    fn example_one() {
        assert_eq!(largest_island(vec![vec![1, 0], vec![0, 1]]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_island(vec![vec![1, 1], vec![1, 0]]), 4);
    }
}
