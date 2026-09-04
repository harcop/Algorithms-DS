/// LeetCode #529 - Minesweeper
fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
    let r = click[0] as usize;
    let c = click[1] as usize;
    if board[r][c] == 'M' {
        board[r][c] = 'X';
        return board;
    }
    let m = board.len();
    let n = board[0].len();
    fn count_mines(board: &[Vec<char>], r: usize, c: usize) -> u8 {
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        let mut cnt = 0u8;
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < m && nc >= 0 && nc < n && board[nr as usize][nc as usize] == 'M' {
                    cnt += 1;
                }
            }
        }
        cnt
    }
    fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize) {
        if board[r][c] != 'E' {
            return;
        }
        let mines = count_mines(board, r, c);
        if mines > 0 {
            board[r][c] = (b'0' + mines) as char;
            return;
        }
        board[r][c] = 'B';
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < m && nc >= 0 && nc < n {
                    dfs(board, nr as usize, nc as usize);
                }
            }
        }
    }
    let _ = (m, n);
    dfs(&mut board, r, c);
    board
}

fn main() {
    let board = vec![
        vec!['E', 'E', 'E', 'E', 'E'],
        vec!['E', 'E', 'M', 'E', 'E'],
        vec!['E', 'E', 'E', 'E', 'E'],
        vec!['E', 'E', 'E', 'E', 'E'],
    ];
    println!("{:?}", update_board(board, vec![3, 0]));
}

#[cfg(test)]
mod tests {
    use super::update_board;

    #[test]
    fn example_one() {
        let board = vec![
            vec!['E', 'E', 'E', 'E', 'E'],
            vec!['E', 'E', 'M', 'E', 'E'],
            vec!['E', 'E', 'E', 'E', 'E'],
            vec!['E', 'E', 'E', 'E', 'E'],
        ];
        let expected = vec![
            vec!['B', '1', 'E', '1', 'B'],
            vec!['B', '1', 'M', '1', 'B'],
            vec!['B', '1', '1', '1', 'B'],
            vec!['B', 'B', 'B', 'B', 'B'],
        ];
        assert_eq!(update_board(board, vec![3, 0]), expected);
    }

    #[test]
    fn example_two() {
        let board = vec![
            vec!['B', '1', 'E', '1', 'B'],
            vec!['B', '1', 'M', '1', 'B'],
            vec!['B', '1', '1', '1', 'B'],
            vec!['B', 'B', 'B', 'B', 'B'],
        ];
        let expected = vec![
            vec!['B', '1', 'E', '1', 'B'],
            vec!['B', '1', 'X', '1', 'B'],
            vec!['B', '1', '1', '1', 'B'],
            vec!['B', 'B', 'B', 'B', 'B'],
        ];
        assert_eq!(update_board(board, vec![1, 2]), expected);
    }
}
