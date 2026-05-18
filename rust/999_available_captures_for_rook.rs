/// LeetCode #999 - Available Captures for Rook
fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
    let n = board.len();
    let mut r = 0usize;
    let mut c = 0usize;
    for i in 0..n {
        for j in 0..n {
            if board[i][j] == 'R' {
                r = i;
                c = j;
            }
        }
    }
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut count = 0i32;
    for (dr, dc) in dirs {
        let mut i = r as i32 + dr;
        let mut j = c as i32 + dc;
        while i >= 0 && j >= 0 && (i as usize) < n && (j as usize) < n {
            let ui = i as usize;
            let uj = j as usize;
            if board[ui][uj] == 'p' {
                count += 1;
            }
            if board[ui][uj] != '.' {
                break;
            }
            i += dr;
            j += dc;
        }
    }
    count
}

fn main() {
    let board = vec![
        vec!['.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'p', '.', '.', '.'],
        vec!['.', '.', '.', 'R', '.', '.', 'p'],
        vec!['.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.'],
    ];
    println!("{}", num_rook_captures(board));
}

#[cfg(test)]
mod tests {
    use super::num_rook_captures;

    #[test]
    fn example_one() {
        let board = vec![
            vec!['.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.'],
            vec!['.', '.', '.', 'R', '.', '.', 'p'],
            vec!['.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(num_rook_captures(board), 3);
    }
}
