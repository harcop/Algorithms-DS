/// LeetCode #3552 - Grid Teleportation Traversal
use std::collections::{HashMap, VecDeque};

fn min_moves(matrix: Vec<String>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let grid: Vec<Vec<u8>> = matrix.iter().map(|s| s.as_bytes().to_vec()).collect();
    let mut portals: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..m {
        for j in 0..n {
            let c = grid[i][j];
            if c.is_ascii_uppercase() {
                portals.entry(c).or_default().push((i, j));
            }
        }
    }
    const INF: i32 = i32::MAX / 2;
    let mut dist = vec![vec![INF; n]; m];
    dist[0][0] = 0;
    let mut q = VecDeque::new();
    q.push_back((0usize, 0usize));
    let dirs = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];
    while let Some((i, j)) = q.pop_front() {
        let d = dist[i][j];
        if i == m - 1 && j == n - 1 {
            return d;
        }
        let c = grid[i][j];
        if let Some(pts) = portals.remove(&c) {
            for (x, y) in pts {
                if d < dist[x][y] {
                    dist[x][y] = d;
                    q.push_front((x, y));
                }
            }
        }
        for (dx, dy) in dirs {
            let x = i as i32 + dx;
            let y = j as i32 + dy;
            if x >= 0 && y >= 0 {
                let x = x as usize;
                let y = y as usize;
                if x < m && y < n && grid[x][y] != b'#' && d + 1 < dist[x][y] {
                    dist[x][y] = d + 1;
                    q.push_back((x, y));
                }
            }
        }
    }
    -1
}

fn main() {
    println!(
        "{}",
        min_moves(vec!["A..".into(), ".A.".into(), "...".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::min_moves;

    #[test]
    fn example1() {
        assert_eq!(
            min_moves(vec!["A..".into(), ".A.".into(), "...".into()]),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_moves(vec![
                ".#...".into(),
                ".#.#.".into(),
                ".#.#.".into(),
                "...#.".into(),
            ]),
            13
        );
    }
}
