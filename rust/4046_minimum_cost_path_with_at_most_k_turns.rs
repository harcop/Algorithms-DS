/// LeetCode #4046 - Minimum Cost Path With At Most K Turns
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    if m == 1 && n == 1 {
        return grid[0][0];
    }
    let k = k as usize;
    let dr = [0i32, 1, 0, -1];
    let dc = [1i32, 0, -1, 0];
    let idx = |r: usize, c: usize, d: usize, t: usize| ((r * n + c) * 4 + d) * (k + 1) + t;
    let mut dist = vec![i64::MAX; m * n * 4 * (k + 1)];
    let mut heap = BinaryHeap::new();
    let start = grid[0][0] as i64;
    for d in 0..4 {
        let nr = dr[d];
        let nc = dc[d];
        if nr < 0 || nc < 0 || nr >= m as i32 || nc >= n as i32 {
            continue;
        }
        let (r, c) = (nr as usize, nc as usize);
        let cost = start + grid[r][c] as i64;
        dist[idx(r, c, d, 0)] = cost;
        heap.push(Reverse((cost, r, c, d, 0usize)));
    }
    while let Some(Reverse((cost, r, c, d, t))) = heap.pop() {
        if cost != dist[idx(r, c, d, t)] {
            continue;
        }
        if r + 1 == m && c + 1 == n {
            return cost as i32;
        }
        for nd in 0..4 {
            let nr = r as i32 + dr[nd];
            let nc = c as i32 + dc[nd];
            if nr < 0 || nc < 0 || nr >= m as i32 || nc >= n as i32 {
                continue;
            }
            let nt = if nd == d { t } else { t + 1 };
            if nt > k {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            let ncost = cost + grid[nr][nc] as i64;
            let id = idx(nr, nc, nd, nt);
            if ncost < dist[id] {
                dist[id] = ncost;
                heap.push(Reverse((ncost, nr, nc, nd, nt)));
            }
        }
    }
    -1
}

fn main() {
    println!("{}", min_cost(vec![vec![2, 7, 3], vec![1, 4, 5]], 1));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example1() {
        assert_eq!(min_cost(vec![vec![2, 7, 3], vec![1, 4, 5]], 1), 12);
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_cost(vec![vec![4, 1, 9], vec![3, 2, 5], vec![4, 8, 6]], 2),
            20
        );
    }

    #[test]
    fn example3() {
        assert_eq!(min_cost(vec![vec![1, 9], vec![3, 4]], 0), -1);
    }
}
