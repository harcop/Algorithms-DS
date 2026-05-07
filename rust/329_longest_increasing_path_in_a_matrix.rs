/// LeetCode #329 - Longest Increasing Path in a Matrix
fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    let rows = matrix.len();
    if rows == 0 {
        return 0;
    }
    let cols = matrix[0].len();
    let mut memo = vec![vec![0i32; cols]; rows];
    let dirs = [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)];

    fn dfs(
        r: usize,
        c: usize,
        matrix: &[Vec<i32>],
        memo: &mut [Vec<i32>],
        dirs: &[(i32, i32)],
    ) -> i32 {
        if memo[r][c] > 0 {
            return memo[r][c];
        }
        let mut best = 1;
        for &(dr, dc) in dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nc < 0 || nr >= matrix.len() as i32 || nc >= matrix[0].len() as i32 {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if matrix[nr][nc] > matrix[r][c] {
                best = best.max(1 + dfs(nr, nc, matrix, memo, dirs));
            }
        }
        memo[r][c] = best;
        best
    }

    let mut ans = 0;
    for r in 0..rows {
        for c in 0..cols {
            ans = ans.max(dfs(r, c, &matrix, &mut memo, &dirs));
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::longest_increasing_path;

    #[test]
    fn examples() {
        assert_eq!(
            longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]),
            4
        );
        assert_eq!(
            longest_increasing_path(vec![vec![3, 4, 5], vec![3, 2, 6]]),
            4
        );
    }
}
