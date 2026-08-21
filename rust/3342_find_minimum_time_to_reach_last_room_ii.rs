/// LeetCode #3342 - Find Minimum Time to Reach Last Room II
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    let n = move_time.len();
    let m = move_time[0].len();
    let mut dist = vec![vec![i64::MAX; m]; n];
    dist[0][0] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0i64, 0usize, 0usize)));
    let dirs = [(0isize, 1isize), (0, -1), (1, 0), (-1, 0)];
    while let Some(Reverse((d, i, j))) = pq.pop() {
        if i == n - 1 && j == m - 1 {
            return d as i32;
        }
        if d > dist[i][j] {
            continue;
        }
        for &(di, dj) in &dirs {
            let x = i as isize + di;
            let y = j as isize + dj;
            if x >= 0 && y >= 0 && (x as usize) < n && (y as usize) < m {
                let x = x as usize;
                let y = y as usize;
                let t = (move_time[x][y] as i64).max(dist[i][j]) + (i + j) as i64 % 2 + 1;
                if dist[x][y] > t {
                    dist[x][y] = t;
                    pq.push(Reverse((t, x, y)));
                }
            }
        }
    }
    -1
}

fn main() {
    println!("{}", min_time_to_reach(vec![vec![0, 4], vec![4, 4]]));
}

#[cfg(test)]
mod tests {
    use super::min_time_to_reach;

    #[test]
    fn example1() {
        assert_eq!(min_time_to_reach(vec![vec![0, 4], vec![4, 4]]), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_time_to_reach(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0]]),
            6
        );
    }

    #[test]
    fn example3() {
        assert_eq!(min_time_to_reach(vec![vec![0, 1], vec![1, 2]]), 4);
    }
}
