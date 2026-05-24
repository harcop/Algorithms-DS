/// LeetCode #1275 - Find Winner on a Tic Tac Toe Game
fn tictactoe(moves: Vec<Vec<i32>>) -> String {
    let mut board = [[0i8; 3]; 3];
    for (i, m) in moves.iter().enumerate() {
        let p = if i % 2 == 0 { 1 } else { -1 };
        board[m[0] as usize][m[1] as usize] = p;
        if wins(&board, p) {
            return if p == 1 { "A" } else { "B" }.into();
        }
    }
    if moves.len() == 9 {
        "Draw".into()
    } else {
        "Pending".into()
    }
}

fn wins(board: &[[i8; 3]; 3], p: i8) -> bool {
    for i in 0..3 {
        if board[i][0] == p && board[i][1] == p && board[i][2] == p {
            return true;
        }
        if board[0][i] == p && board[1][i] == p && board[2][i] == p {
            return true;
        }
    }
    board[0][0] == p && board[1][1] == p && board[2][2] == p
        || board[0][2] == p && board[1][1] == p && board[2][0] == p
}

fn main() {
    println!(
        "{}",
        tictactoe(vec![vec![0, 0], vec![2, 0], vec![1, 1], vec![2, 1], vec![2, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::tictactoe;

    #[test]
    fn example_one() {
        assert_eq!(
            tictactoe(vec![vec![0, 0], vec![2, 0], vec![1, 1], vec![2, 1], vec![2, 2]]),
            "A"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            tictactoe(vec![
                vec![0, 0],
                vec![1, 1],
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![2, 0],
            ]),
            "B"
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            tictactoe(vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 0],
                vec![1, 0],
                vec![1, 2],
                vec![2, 1],
                vec![0, 1],
                vec![0, 2],
                vec![2, 2],
            ]),
            "Draw"
        );
    }
}
