/// LeetCode #2577 - Minimum Time to Visit a Cell In a Grid
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
    if grid[0][1] > 1 && grid[1][0] > 1 {
        return -1;
    }
    let m = grid.len();
    let n = grid[0].len();
    let mut dist = vec![vec![i32::MAX; n]; m];
    dist[0][0] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0i32, 0usize, 0usize)));
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some(Reverse((t, i, j))) = pq.pop() {
        if i == m - 1 && j == n - 1 {
            return t;
        }
        if t > dist[i][j] {
            continue;
        }
        for &(di, dj) in &dirs {
            let x = i as i32 + di;
            let y = j as i32 + dj;
            if x < 0 || y < 0 || x as usize >= m || y as usize >= n {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            let mut nt = t + 1;
            if nt < grid[x][y] {
                nt = grid[x][y] + (grid[x][y] - nt) % 2;
            }
            if nt < dist[x][y] {
                dist[x][y] = nt;
                pq.push(Reverse((nt, x, y)));
            }
        }
    }
    -1
}

fn main() {
    println!(
        "{}",
        minimum_time(vec![
            vec![0, 1, 3, 2],
            vec![5, 1, 2, 5],
            vec![4, 3, 8, 6]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_time;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_time(vec![
                vec![0, 1, 3, 2],
                vec![5, 1, 2, 5],
                vec![4, 3, 8, 6]
            ]),
            7
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_time(vec![vec![0, 2, 4], vec![3, 2, 1], vec![1, 0, 4]]),
            -1
        );
    }
}
