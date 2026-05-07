/// LeetCode #407 - Trapping Rain Water II (min-height BFS expansion)
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
    let rows = height_map.len();
    if rows == 0 {
        return 0;
    }
    let cols = height_map[0].len();
    let mut pq: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
    let mut visited = vec![vec![false; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            if r == 0 || c == 0 || r + 1 == rows || c + 1 == cols {
                pq.push(Reverse((height_map[r][c], r, c)));
                visited[r][c] = true;
            }
        }
    }

    let mut ans = 0i32;
    let dirs = [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)];
    while let Some(Reverse((h, r, c))) = pq.pop() {
        for &(dr, dc) in &dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if visited[nr][nc] {
                continue;
            }
            visited[nr][nc] = true;
            let nh = height_map[nr][nc];
            ans += (h.max(nh) - nh).max(0);
            pq.push(Reverse((h.max(nh), nr, nc)));
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        trap_rain_water(vec![vec![1, 4, 3, 1, 3, 2], vec![3, 2, 1, 3, 2, 4], vec![2, 3, 3, 2, 3, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc() {
        assert_eq!(
            trap_rain_water(vec![vec![1, 4, 3, 1, 3, 2], vec![3, 2, 1, 3, 2, 4], vec![2, 3, 3, 2, 3, 1]]),
            4
        );
    }
}
