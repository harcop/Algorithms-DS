/// LeetCode #2503 - Maximum Number of Points From Grid Queries
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let m = grid.len();
    let n = grid[0].len();
    let k = queries.len();
    let mut qs: Vec<(i32, usize)> = queries.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
    qs.sort_unstable();

    let mut ans = vec![0; k];
    let mut vis = vec![vec![false; n]; m];
    vis[0][0] = true;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((grid[0][0], 0usize, 0usize)));
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut cnt = 0;

    for (v, idx) in qs {
        while let Some(Reverse((val, _, _))) = heap.peek() {
            if *val >= v {
                break;
            }
            let Reverse((_, i, j)) = heap.pop().unwrap();
            cnt += 1;
            for &(di, dj) in &dirs {
                let x = i as i32 + di;
                let y = j as i32 + dj;
                if x >= 0 && x < m as i32 && y >= 0 && y < n as i32 {
                    let (x, y) = (x as usize, y as usize);
                    if !vis[x][y] {
                        vis[x][y] = true;
                        heap.push(Reverse((grid[x][y], x, y)));
                    }
                }
            }
        }
        ans[idx] = cnt;
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        max_points(vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]], vec![5, 6, 2])
    );
}

#[cfg(test)]
mod tests {
    use super::max_points;

    #[test]
    fn example_one() {
        assert_eq!(
            max_points(vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]], vec![5, 6, 2]),
            vec![5, 8, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_points(vec![vec![5, 2, 1], vec![1, 1, 2]], vec![3]),
            vec![0]
        );
    }
}
