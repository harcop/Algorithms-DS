/// LeetCode #778 - Swim in Rising Water
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut pq: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
    let mut seen = vec![vec![false; n]; n];
    pq.push(Reverse((grid[0][0], 0, 0)));
    seen[0][0] = true;
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some(Reverse((t, r, c))) = pq.pop() {
        if r == n - 1 && c == n - 1 {
            return t;
        }
        for (dr, dc) in dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nc < 0 || nr >= n as i32 || nc >= n as i32 {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if seen[nr][nc] {
                continue;
            }
            seen[nr][nc] = true;
            let nt = t.max(grid[nr][nc]);
            pq.push(Reverse((nt, nr, nc)));
        }
    }
    0
}

fn main() {
    let g = vec![vec![0, 2], vec![1, 3]];
    println!("{}", swim_in_water(g));
}

#[cfg(test)]
mod tests {
    use super::swim_in_water;

    #[test]
    fn example_one() {
        assert_eq!(swim_in_water(vec![vec![0, 2], vec![1, 3]]), 3);
    }
}
