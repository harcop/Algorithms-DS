/// LeetCode #1631 - Path With Minimum Effort
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let n = heights.len();
    let m = heights[0].len();
    let mut dist = vec![vec![i32::MAX; m]; n];
    dist[0][0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, 0usize, 0usize)));
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some(Reverse((d, i, j))) = heap.pop() {
        if d > dist[i][j] { continue; }
        if i == n - 1 && j == m - 1 { return d; }
        for (di, dj) in dirs {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni < 0 || nj < 0 || ni as usize >= n || nj as usize >= m { continue; }
            let ni = ni as usize;
            let nj = nj as usize;
            let nd = d.max((heights[i][j] - heights[ni][nj]).abs());
            if nd < dist[ni][nj] {
                dist[ni][nj] = nd;
                heap.push(Reverse((nd, ni, nj)));
            }
        }
    }
    0
}
fn main() { println!("{}", minimum_effort_path(vec![vec![1,2,2],vec![3,8,2],vec![5,3,5]])); }
#[cfg(test)]
mod tests {
    use super::minimum_effort_path;
    #[test]
    fn example_one() { assert_eq!(minimum_effort_path(vec![vec![1,2,2],vec![3,8,2],vec![5,3,5]]), 2); }
}