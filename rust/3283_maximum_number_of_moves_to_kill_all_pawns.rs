/// LeetCode #3283 - Maximum Number of Moves to Kill All Pawns
use std::collections::VecDeque;

fn max_moves(kx: i32, ky: i32, mut positions: Vec<Vec<i32>>) -> i32 {
    let n = positions.len();
    const M: usize = 50;
    let dx = [1i32, 1, 2, 2, -1, -1, -2, -2];
    let dy = [2i32, -2, 1, -1, 2, -2, 1, -1];
    positions.push(vec![kx, ky]);
    let mut dist = vec![vec![vec![-1i32; M]; M]; n + 1];
    for i in 0..=n {
        let x = positions[i][0] as usize;
        let y = positions[i][1] as usize;
        dist[i][x][y] = 0;
        let mut q = VecDeque::new();
        q.push_back((x, y));
        while let Some((x1, y1)) = q.pop_front() {
            let step = dist[i][x1][y1] + 1;
            for j in 0..8 {
                let x2 = x1 as i32 + dx[j];
                let y2 = y1 as i32 + dy[j];
                if x2 >= 0 && x2 < M as i32 && y2 >= 0 && y2 < M as i32 {
                    let x2 = x2 as usize;
                    let y2 = y2 as usize;
                    if dist[i][x2][y2] == -1 {
                        dist[i][x2][y2] = step;
                        q.push_back((x2, y2));
                    }
                }
            }
        }
    }
    let mut memo = vec![vec![vec![-1i32; 2]; 1 << n]; n + 1];
    fn dfs(
        last: usize,
        state: usize,
        alice: usize,
        n: usize,
        positions: &[Vec<i32>],
        dist: &[Vec<Vec<i32>>],
        memo: &mut [Vec<Vec<i32>>],
    ) -> i32 {
        if state == 0 {
            return 0;
        }
        if memo[last][state][alice] != -1 {
            return memo[last][state][alice];
        }
        let mut res = if alice == 1 { 0 } else { i32::MAX / 4 };
        for i in 0..n {
            if (state >> i) & 1 == 1 {
                let x = positions[i][0] as usize;
                let y = positions[i][1] as usize;
                let t = dfs(i, state ^ (1 << i), alice ^ 1, n, positions, dist, memo)
                    + dist[last][x][y];
                if alice == 1 {
                    res = res.max(t);
                } else {
                    res = res.min(t);
                }
            }
        }
        memo[last][state][alice] = res;
        res
    }
    dfs(n, (1 << n) - 1, 1, n, &positions, &dist, &mut memo)
}

fn main() {
    println!("{}", max_moves(1, 1, vec![vec![0, 0]]));
}

#[cfg(test)]
mod tests {
    use super::max_moves;

    #[test]
    fn example1() {
        assert_eq!(max_moves(1, 1, vec![vec![0, 0]]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_moves(0, 2, vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            8
        );
    }

    #[test]
    fn example3() {
        assert_eq!(max_moves(0, 0, vec![vec![1, 2], vec![2, 4]]), 3);
    }
}
