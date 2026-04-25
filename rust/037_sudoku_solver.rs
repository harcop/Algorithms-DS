/// LeetCode #37 - Sudoku Solver
fn solve_sudoku(board: &mut [Vec<char>]) {
    fn is_valid(board: &[Vec<char>], row: usize, col: usize, digit: u8) -> bool {
        let ch = (b'0' + digit) as char;
        for c in 0..9 {
            if board[row][c] == ch {
                return false;
            }
        }
        for r in 0..9 {
            if board[r][col] == ch {
                return false;
            }
        }
        let br = 3 * (row / 3);
        let bc = 3 * (col / 3);
        for r in 0..3 {
            for c in 0..3 {
                if board[br + r][bc + c] == ch {
                    return false;
                }
            }
        }
        true
    }

    fn backtrack(board: &mut [Vec<char>], cell: usize) -> bool {
        if cell == 81 {
            return true;
        }
        let r = cell / 9;
        let c = cell % 9;
        if board[r][c] != '.' {
            return backtrack(board, cell + 1);
        }
        for d in 1..=9u8 {
            if is_valid(board, r, c, d) {
                board[r][c] = (b'0' + d) as char;
                if backtrack(board, cell + 1) {
                    return true;
                }
                board[r][c] = '.';
            }
        }
        false
    }

    backtrack(board, 0);
}

fn main() {
    let mut board: Vec<Vec<char>> = [
        "53..7....", "6..195...", ".98....6.", "8...6...3", "4..8.3..1", "7...2...6", ".6....28.",
        "...419..5", "....8..79",
    ]
    .iter()
    .map(|r| r.chars().collect())
    .collect();
    solve_sudoku(&mut board);
    for row in &board {
        println!("{}", row.iter().collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use super::solve_sudoku;

    #[test]
    fn example() {
        let solution: [[char; 9]; 9] = [
            ['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            ['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            ['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            ['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            ['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            ['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            ['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            ['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            ['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        let mut board: Vec<Vec<char>> = [
            "53..7....", "6..195...", ".98....6.", "8...6...3", "4..8.3..1", "7...2...6", ".6....28.",
            "...419..5", "....8..79",
        ]
        .iter()
        .map(|r| r.chars().collect())
        .collect();

        solve_sudoku(&mut board);
        for r in 0..9 {
            for c in 0..9 {
                assert_eq!(board[r][c], solution[r][c]);
            }
        }
    }
}
