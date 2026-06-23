/// LeetCode #2056 - Number of Valid Move Combinations On Chessboard
const M: usize = 9;

fn rook_dirs() -> [(i32, i32); 4] {
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
}

fn bishop_dirs() -> [(i32, i32); 4] {
    [(1, 1), (1, -1), (-1, 1), (-1, -1)]
}

fn get_dirs(piece: &str) -> Vec<(i32, i32)> {
    match piece.as_bytes()[0] {
        b'r' => rook_dirs().to_vec(),
        b'b' => bishop_dirs().to_vec(),
        _ => {
            let mut dirs = rook_dirs().to_vec();
            dirs.extend_from_slice(&bishop_dirs());
            dirs
        }
    }
}

fn check_stop(dist: &[Vec<Vec<i32>>], i: usize, x: usize, y: usize, t: i32) -> bool {
    (0..i).all(|j| dist[j][x][y] < t)
}

fn check_pass(
    dist: &[Vec<Vec<i32>>],
    end: &[(i32, i32, i32)],
    i: usize,
    x: usize,
    y: usize,
    t: i32,
) -> bool {
    for j in 0..i {
        if dist[j][x][y] == t {
            return false;
        }
        if end[j].0 == x as i32 && end[j].1 == y as i32 && end[j].2 <= t {
            return false;
        }
    }
    true
}

fn dfs(
    i: usize,
    n: usize,
    pieces: &[String],
    positions: &[Vec<i32>],
    dist: &mut [Vec<Vec<i32>>],
    end: &mut [(i32, i32, i32)],
    ans: &mut i32,
) {
    if i >= n {
        *ans += 1;
        return;
    }
    let x = positions[i][0] as usize;
    let y = positions[i][1] as usize;

    dist[i] = vec![vec![-1; M]; M];
    dist[i][x][y] = 0;
    end[i] = (x as i32, y as i32, 0);
    if check_stop(dist, i, x, y, 0) {
        dfs(i + 1, n, pieces, positions, dist, end, ans);
    }

    for (dx, dy) in get_dirs(&pieces[i]) {
        dist[i] = vec![vec![-1; M]; M];
        dist[i][x][y] = 0;
        let mut nx = x as i32 + dx;
        let mut ny = y as i32 + dy;
        let mut nt = 1i32;
        while (1..M as i32).contains(&nx)
            && (1..M as i32).contains(&ny)
            && check_pass(dist, end, i, nx as usize, ny as usize, nt)
        {
            dist[i][nx as usize][ny as usize] = nt;
            end[i] = (nx, ny, nt);
            if check_stop(dist, i, nx as usize, ny as usize, nt) {
                dfs(i + 1, n, pieces, positions, dist, end, ans);
            }
            nx += dx;
            ny += dy;
            nt += 1;
        }
    }
}

fn count_combinations(pieces: Vec<String>, positions: Vec<Vec<i32>>) -> i32 {
    let n = pieces.len();
    let mut dist = vec![vec![vec![-1; M]; M]; n];
    let mut end = vec![(0, 0, 0); n];
    let mut ans = 0;
    dfs(0, n, &pieces, &positions, &mut dist, &mut end, &mut ans);
    ans
}

fn main() {
    println!(
        "{}",
        count_combinations(vec!["rook".into()], vec![vec![1, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::count_combinations;

    #[test]
    fn example_one() {
        assert_eq!(count_combinations(vec!["rook".into()], vec![vec![1, 1]]), 15);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_combinations(vec!["queen".into()], vec![vec![1, 1]]), 22);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            count_combinations(vec!["bishop".into()], vec![vec![4, 3]]),
            12
        );
    }
}
