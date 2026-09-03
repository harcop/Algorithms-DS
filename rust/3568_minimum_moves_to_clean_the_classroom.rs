/// LeetCode #3568 - Minimum Moves to Clean the Classroom
use std::collections::VecDeque;

fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
    let m = classroom.len();
    let n = classroom[0].len();
    let e_max = energy as usize;
    let grid: Vec<Vec<u8>> = classroom.iter().map(|s| s.as_bytes().to_vec()).collect();
    let mut d = vec![vec![0usize; n]; m];
    let mut sx = 0usize;
    let mut sy = 0usize;
    let mut cnt = 0usize;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == b'S' {
                sx = i;
                sy = j;
            } else if grid[i][j] == b'L' {
                d[i][j] = cnt;
                cnt += 1;
            }
        }
    }
    if cnt == 0 {
        return 0;
    }
    let masks = 1usize << cnt;
    let mut vis = vec![false; m * n * (e_max + 1) * masks];
    let id = |i: usize, j: usize, en: usize, mask: usize| ((i * n + j) * (e_max + 1) + en) * masks + mask;
    let full = masks - 1;
    vis[id(sx, sy, e_max, full)] = true;
    let mut q = VecDeque::from([(sx, sy, e_max, full)]);
    let dirs = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];
    let mut ans = 0;
    while !q.is_empty() {
        for _ in 0..q.len() {
            let (i, j, cur, mask) = q.pop_front().unwrap();
            if mask == 0 {
                return ans;
            }
            if cur == 0 {
                continue;
            }
            for &(dx, dy) in &dirs {
                let nx = i as i32 + dx;
                let ny = j as i32 + dy;
                if nx < 0 || ny < 0 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if nx >= m || ny >= n || grid[nx][ny] == b'X' {
                    continue;
                }
                let nxt_e = if grid[nx][ny] == b'R' { e_max } else { cur - 1 };
                let mut nxt_mask = mask;
                if grid[nx][ny] == b'L' {
                    nxt_mask &= !(1 << d[nx][ny]);
                }
                let idx = id(nx, ny, nxt_e, nxt_mask);
                if !vis[idx] {
                    vis[idx] = true;
                    q.push_back((nx, ny, nxt_e, nxt_mask));
                }
            }
        }
        ans += 1;
    }
    -1
}

fn main() {
    println!("{}", min_moves(vec!["S.".into(), "XL".into()], 2));
}

#[cfg(test)]
mod tests {
    use super::min_moves;

    #[test]
    fn example1() {
        assert_eq!(min_moves(vec!["S.".into(), "XL".into()], 2), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_moves(vec!["LS".into(), "RL".into()], 4), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(min_moves(vec!["L.S".into(), "RXL".into()], 3), -1);
    }
}
