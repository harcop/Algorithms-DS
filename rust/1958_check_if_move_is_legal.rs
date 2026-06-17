/// LeetCode #1958 - Check if Move is Legal
fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: String) -> bool {
    let color = color.as_bytes()[0] as char;
    for a in -1..=1 {
        for b in -1..=1 {
            if a == 0 && b == 0 {
                continue;
            }
            let mut i = r_move;
            let mut j = c_move;
            let mut cnt = 0;
            loop {
                cnt += 1;
                i += a;
                j += b;
                if i < 0 || j < 0 || i >= 8 || j >= 8 {
                    break;
                }
                let cell = board[i as usize][j as usize];
                if cnt > 1 && cell == color {
                    return true;
                }
                if cell == color || cell == '.' {
                    break;
                }
            }
        }
    }
    false
}

fn main() {
    let board = vec![
        vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
        vec!['W', 'B', 'B', '.', 'W', 'W', 'W', 'B'],
        vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
    ];
    println!("{}", check_move(board, 4, 3, "B".into()));
}

#[cfg(test)]
mod tests {
    use super::check_move;

    #[test]
    fn example_one() {
        let board = vec![
            vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
            vec!['W', 'B', 'B', '.', 'W', 'W', 'W', 'B'],
            vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
        ];
        assert!(check_move(board, 4, 3, "B".into()));
    }

    #[test]
    fn example_two() {
        let board = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', 'B', '.', '.', 'W', '.', '.', '.'],
            vec!['.', '.', 'W', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'W', 'B', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', 'B', 'W', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', 'W', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', 'B'],
        ];
        assert!(!check_move(board, 4, 4, "W".into()));
    }
}
