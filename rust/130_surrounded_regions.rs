/// LeetCode #130 - Surrounded Regions
fn solve(board: &mut Vec<Vec<char>>) {
    if board.is_empty() {
        return;
    }
    let m = board.len();
    let n = board[0].len();

    fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if i >= board.len() || j >= board[0].len() || board[i][j] != 'O' {
            return;
        }
        board[i][j] = '#';
        if i > 0 {
            dfs(board, i - 1, j);
        }
        if i + 1 < board.len() {
            dfs(board, i + 1, j);
        }
        if j > 0 {
            dfs(board, i, j - 1);
        }
        if j + 1 < board[0].len() {
            dfs(board, i, j + 1);
        }
    }

    for i in 0..m {
        dfs(board, i, 0);
        dfs(board, i, n - 1);
    }
    for j in 0..n {
        dfs(board, 0, j);
        dfs(board, m - 1, j);
    }

    for row in board.iter_mut() {
        for c in row.iter_mut() {
            *c = match *c {
                '#' => 'O',
                'O' => 'X',
                _ => *c,
            }
        }
    }
}

fn main() {
    let mut b = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    solve(&mut b);
    println!("{b:?}");
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example_one() {
        let mut b = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        solve(&mut b);
        assert_eq!(
            b,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        );
    }
}
