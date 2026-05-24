/// LeetCode #1301 - Number of Paths with Max Score
const MOD: i64 = 1_000_000_007;

fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
    let n = board.len();
    let mut score = vec![vec![-1i64; n]; n];
    let mut ways = vec![vec![0i64; n]; n];
    score[n - 1][n - 1] = 0;
    ways[n - 1][n - 1] = 1;
    for i in (0..n).rev() {
        for j in (0..n).rev() {
            if board[i].as_bytes()[j] == b'X' {
                continue;
            }
            for (di, dj) in [(1, 0), (0, 1), (1, 1)] {
                let ni = i + di;
                let nj = j + dj;
                if ni >= n || nj >= n || score[ni][nj] < 0 {
                    continue;
                }
                let add = match board[i].as_bytes()[j] {
                    b'S' | b'E' => 0,
                    c => (c - b'0') as i64,
                };
                let ns = score[ni][nj] + add;
                if ns > score[i][j] {
                    score[i][j] = ns;
                    ways[i][j] = ways[ni][nj];
                } else if ns == score[i][j] {
                    ways[i][j] = (ways[i][j] + ways[ni][nj]) % MOD;
                }
            }
        }
    }
    if score[0][0] < 0 {
        vec![0, 0]
    } else {
        vec![score[0][0] as i32, ways[0][0] as i32]
    }
}

fn main() {
    println!("{:?}", paths_with_max_score(vec!["E23".to_string(), "2X2".to_string(), "12S".to_string()]));
}

#[cfg(test)]
mod tests {
    use super::paths_with_max_score;

    #[test]
    fn example_one() {
        assert_eq!(
            paths_with_max_score(vec!["E23".to_string(), "2X2".to_string(), "12S".to_string()]),
            vec![7, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(paths_with_max_score(vec!["E12".to_string(), "1X1".to_string(), "21S".to_string()]), vec![4, 2]);
    }
}
