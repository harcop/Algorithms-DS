/// LeetCode #317 - Shortest Distance from All Buildings
fn shortest_distance(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    if rows == 0 {
        return -1;
    }
    let cols = grid[0].len();
    let mut dist_sum = vec![vec![0i32; cols]; rows];
    let mut reach = vec![vec![0i32; cols]; rows];
    let mut buildings = 0i32;

    let dirs = [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)];

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != 1 {
                continue;
            }
            buildings += 1;
            let mut q = std::collections::VecDeque::new();
            q.push_back((r, c, 0));
            let mut local = vec![vec![false; cols]; rows];
            local[r][c] = true;
            while let Some((cr, cc, d)) = q.pop_front() {
                for &(dr, dc) in &dirs {
                    let nr = cr as i32 + dr;
                    let nc = cc as i32 + dc;
                    if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if grid[nr][nc] != 0 || local[nr][nc] {
                        continue;
                    }
                    local[nr][nc] = true;
                    let nd = d + 1;
                    dist_sum[nr][nc] += nd;
                    reach[nr][nc] += 1;
                    q.push_back((nr, nc, nd));
                }
            }
        }
    }

    let mut best = i32::MAX;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 0 && reach[r][c] == buildings {
                best = best.min(dist_sum[r][c]);
            }
        }
    }
    if best == i32::MAX {
        -1
    } else {
        best
    }
}

fn main() {
    println!(
        "{}",
        shortest_distance(vec![vec![1, 0, 2, 0, 1], vec![0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::shortest_distance;

    #[test]
    fn example() {
        assert_eq!(
            shortest_distance(vec![vec![1, 0, 2, 0, 1], vec![0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0]]),
            7
        );
    }
}
