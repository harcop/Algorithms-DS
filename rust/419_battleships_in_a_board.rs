/// LeetCode #419 - Battleships in a Board
fn count_battleships(board: Vec<Vec<char>>) -> i32 {
    let m = board.len();
    if m == 0 {
        return 0;
    }
    let n = board[0].len();
    let mut c = 0i32;
    for i in 0..m {
        for j in 0..n {
            if board[i][j] != 'X' {
                continue;
            }
            if i > 0 && board[i - 1][j] == 'X' {
                continue;
            }
            if j > 0 && board[i][j - 1] == 'X' {
                continue;
            }
            c += 1;
        }
    }
    c
}

fn main() {
    println!(
        "{}",
        count_battleships(vec![
            vec!['X', '.', '.', 'X'],
            vec!['.', '.', '.', 'X'],
            vec!['.', '.', '.', 'X'],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::count_battleships;

    #[test]
    fn example_one() {
        assert_eq!(
            count_battleships(vec![
                vec!['X', '.', '.', 'X'],
                vec!['.', '.', '.', 'X'],
                vec!['.', '.', '.', 'X'],
            ]),
            2
        );
    }
}
