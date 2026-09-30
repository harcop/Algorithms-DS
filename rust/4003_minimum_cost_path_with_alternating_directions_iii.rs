/// LeetCode #4003 - Minimum Cost Path with Alternating Directions III
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn min_cost(m: i32, n: i32, penalty: Vec<Vec<i32>>) -> i64 {
    let m = m as usize;
    let n = n as usize;
    let inf = i64::MAX / 4;
    let mut dist = vec![vec![[inf; 2]; n]; m];
    dist[0][0][1] = 1;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((1i64, 0usize, 0usize, 1usize)));
    let dirs = [(-1isize, 0isize), (0, 1), (0, -1), (1, 0)];
    while let Some(Reverse((d, i, j, k))) = pq.pop() {
        if i + 1 == m && j + 1 == n {
            return d;
        }
        if d > dist[i][j][k] {
            continue;
        }
        let p = penalty[i][j] as i64;
        let nd = d + p;
        let nk = k ^ 1;
        if nd < dist[i][j][nk] {
            dist[i][j][nk] = nd;
            pq.push(Reverse((nd, i, j, nk)));
        }
        for (idx, (dx, dy)) in dirs.iter().enumerate() {
            let x = i as isize + dx;
            let y = j as isize + dy;
            if x < 0 || y < 0 || x >= m as isize || y >= n as isize {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            let extra = if ((idx & 1) ^ k) == 1 { p } else { 0 };
            let nd = d + (x as i64 + 1) * (y as i64 + 1) + extra;
            if nd < dist[x][y][nk] {
                dist[x][y][nk] = nd;
                pq.push(Reverse((nd, x, y, nk)));
            }
        }
    }
    -1
}

fn main() {
    println!("{}", min_cost(2, 2, vec![vec![5, 3], vec![1, 4]]));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example1() {
        assert_eq!(min_cost(2, 2, vec![vec![5, 3], vec![1, 4]]), 8);
    }

    #[test]
    fn example2() {
        assert_eq!(min_cost(2, 2, vec![vec![0, 7], vec![3, 2]]), 7);
    }

    #[test]
    fn example3() {
        assert_eq!(min_cost(2, 3, vec![vec![8, 0, 9], vec![7, 4, 1]]), 12);
    }
}
