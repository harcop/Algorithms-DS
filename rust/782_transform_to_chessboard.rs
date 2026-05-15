/// LeetCode #782 - Transform to Chessboard
fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
    let n = board.len();
    let mut row_sum = vec![0i32; n];
    let mut col_sum = vec![0i32; n];
    let mut row_xor = vec![0i32; n];
    let mut col_xor = vec![0i32; n];
    for i in 0..n {
        for j in 0..n {
            if board[i][j] != board[0][0] {
                row_xor[i] ^= 1;
                col_xor[j] ^= 1;
            }
            row_sum[i] += board[i][j];
            col_sum[j] += board[i][j];
        }
    }
    let half = n as i32 / 2;
    for i in 0..n {
        if row_sum[i] != half && row_sum[i] != half + 1 {
            return -1;
        }
        if col_sum[i] != half && col_sum[i] != half + 1 {
            return -1;
        }
    }
    let mut r_swaps = 0i32;
    let mut c_swaps = 0i32;
    for i in 0..n {
        if row_xor[i] != (i as i32 & 1) {
            r_swaps += 1;
        }
        if col_xor[i] != (i as i32 & 1) {
            c_swaps += 1;
        }
    }
    if r_swaps % 2 != 0 || c_swaps % 2 != 0 {
        return -1;
    }
    r_swaps / 2 + c_swaps / 2
}

fn main() {
    let b = vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![1, 0, 0, 1], vec![1, 0, 0, 1]];
    println!("{}", moves_to_chessboard(b));
}

#[cfg(test)]
mod tests {
    use super::moves_to_chessboard;

    #[test]
    fn example_one() {
        let b = vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![1, 0, 0, 1], vec![1, 0, 0, 1]];
        assert_eq!(moves_to_chessboard(b), 2);
    }
}
