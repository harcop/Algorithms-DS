/// LeetCode #1034 - Coloring A Border
fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let r0 = row as usize;
    let c0 = col as usize;
    let orig = grid[r0][c0];
    let mut g = grid;
    let mut seen = vec![vec![false; cols]; rows];
    let mut border = Vec::new();
    fn dfs(
        g: &mut [Vec<i32>],
        seen: &mut [Vec<bool>],
        border: &mut Vec<(usize, usize)>,
        r: usize,
        c: usize,
        orig: i32,
    ) {
        seen[r][c] = true;
        let mut is_border = false;
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nc < 0 || nr as usize >= g.len() || nc as usize >= g[0].len() {
                is_border = true;
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if g[nr][nc] != orig {
                is_border = true;
            } else if !seen[nr][nc] {
                dfs(g, seen, border, nr, nc, orig);
            }
        }
        if is_border {
            border.push((r, c));
        }
    }
    dfs(&mut g, &mut seen, &mut border, r0, c0, orig);
    for (r, c) in border {
        g[r][c] = color;
    }
    g
}

fn main() {
    let grid = vec![vec![1, 1], vec![1, 2]];
    println!("{:?}", color_border(grid, 1, 1, 3));
}

#[cfg(test)]
mod tests {
    use super::color_border;

    #[test]
    fn example_one() {
        assert_eq!(
            color_border(vec![vec![1, 1], vec![1, 2]], 1, 1, 3),
            vec![vec![1, 3], vec![1, 3]]
        );
    }
}
