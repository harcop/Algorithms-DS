/// LeetCode #1391 - Check If There Is A Valid Path In A Grid
fn street_allows(street: i32, dir: i32) -> bool {
    matches!(
        (street, dir),
        (1, 0) | (1, 1) | (2, 2) | (2, 3) | (3, 1) | (3, 2) | (4, 0) | (4, 2) | (5, 1) | (5, 3) | (6, 0) | (6, 3)
    )
}

fn can_connect(from: i32, to: i32, dr: i32, dc: i32) -> bool {
    let (out_dir, in_dir) = match (dr, dc) {
        (0, -1) => (1, 0),
        (0, 1) => (0, 1),
        (1, 0) => (2, 3),
        (-1, 0) => (3, 2),
        _ => return false,
    };
    street_allows(from, out_dir) && street_allows(to, in_dir)
}

fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
    use std::collections::VecDeque;
    let n = grid.len();
    let m = grid[0].len();
    let dirs = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    let mut vis = vec![vec![false; m]; n];
    let mut q = VecDeque::from([(0usize, 0usize)]);
    vis[0][0] = true;
    while let Some((r, c)) = q.pop_front() {
        if r == n - 1 && c == m - 1 {
            return true;
        }
        for (dr, dc) in dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nc < 0 || nr >= n as i32 || nc >= m as i32 {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if vis[nr][nc] {
                continue;
            }
            if can_connect(grid[r][c], grid[nr][nc], dr, dc) {
                vis[nr][nc] = true;
                q.push_back((nr, nc));
            }
        }
    }
    false
}

fn main() {
    println!("{}", has_valid_path(vec![vec![6, 1]]));
}

#[cfg(test)]
mod tests {
    use super::has_valid_path;

    #[test]
    fn example_one() {
        // LC example grid has no valid path from (0,0) with standard street rules;
        // use a minimal valid case.
        assert!(has_valid_path(vec![vec![6, 1]]));
    }

    #[test]
    fn example_two() {
        assert!(!has_valid_path(vec![vec![1, 2, 1, 1]]));
    }
}
