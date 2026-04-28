/// LeetCode #51 - N-Queens
fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    let mut board = vec![vec!['.'; n]; n];
    let mut cols = vec![false; n];
    let mut diag1 = vec![false; 2 * n - 1]; // row + col
    let mut diag2 = vec![false; 2 * n - 1]; // row - col + n - 1
    let mut out = Vec::new();

    fn backtrack(
        row: usize,
        n: usize,
        board: &mut [Vec<char>],
        cols: &mut [bool],
        diag1: &mut [bool],
        diag2: &mut [bool],
        out: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            out.push(board.iter().map(|r| r.iter().collect()).collect());
            return;
        }

        for col in 0..n {
            let d1 = row + col;
            let d2 = row + (n - 1 - col);
            if cols[col] || diag1[d1] || diag2[d2] {
                continue;
            }
            cols[col] = true;
            diag1[d1] = true;
            diag2[d2] = true;
            board[row][col] = 'Q';

            backtrack(row + 1, n, board, cols, diag1, diag2, out);

            board[row][col] = '.';
            cols[col] = false;
            diag1[d1] = false;
            diag2[d2] = false;
        }
    }

    backtrack(0, n, &mut board, &mut cols, &mut diag1, &mut diag2, &mut out);
    out
}

fn main() {
    println!("{:?}", solve_n_queens(4));
}

#[cfg(test)]
mod tests {
    use super::solve_n_queens;

    #[test]
    fn example_one() {
        assert_eq!(solve_n_queens(4).len(), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(solve_n_queens(1), vec![vec!["Q".to_string()]]);
    }
}
