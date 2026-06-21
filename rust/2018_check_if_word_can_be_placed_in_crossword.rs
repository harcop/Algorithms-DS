/// LeetCode #2018 - Check if Word Can Be Placed In Crossword
fn place_word_in_crossword(board: Vec<Vec<char>>, word: String) -> bool {
    let word = word.as_bytes();
    let m = board.len();
    let n = board[0].len();
    let k = word.len();

    let check = |mut i: i32, mut j: i32, a: i32, b: i32| -> bool {
        let x = i + a * k as i32;
        let y = j + b * k as i32;
        if x >= 0 && x < m as i32 && y >= 0 && y < n as i32 && board[x as usize][y as usize] != '#'
        {
            return false;
        }
        for &c in word {
            if i < 0
                || i >= m as i32
                || j < 0
                || j >= n as i32
                || (board[i as usize][j as usize] != ' ' && board[i as usize][j as usize] != c as char)
            {
                return false;
            }
            i += a;
            j += b;
        }
        true
    };

    for i in 0..m {
        for j in 0..n {
            let left_to_right =
                (j == 0 || board[i][j - 1] == '#') && check(i as i32, j as i32, 0, 1);
            let right_to_left =
                (j == n - 1 || board[i][j + 1] == '#') && check(i as i32, j as i32, 0, -1);
            let up_to_down =
                (i == 0 || board[i - 1][j] == '#') && check(i as i32, j as i32, 1, 0);
            let down_to_up =
                (i == m - 1 || board[i + 1][j] == '#') && check(i as i32, j as i32, -1, 0);
            if left_to_right || right_to_left || up_to_down || down_to_up {
                return true;
            }
        }
    }
    false
}

fn main() {
    let board = vec![
        vec!['#', ' ', '#'],
        vec![' ', ' ', '#'],
        vec!['#', 'c', ' '],
    ];
    println!("{}", place_word_in_crossword(board, "abc".into()));
}

#[cfg(test)]
mod tests {
    use super::place_word_in_crossword;

    #[test]
    fn example_one() {
        let board = vec![
            vec!['#', ' ', '#'],
            vec![' ', ' ', '#'],
            vec!['#', 'c', ' '],
        ];
        assert!(place_word_in_crossword(board, "abc".into()));
    }

    #[test]
    fn example_two() {
        let board = vec![
            vec![' ', '#', 'a'],
            vec![' ', '#', 'c'],
            vec![' ', '#', 'a'],
        ];
        assert!(!place_word_in_crossword(board, "ac".into()));
    }

    #[test]
    fn example_three() {
        let board = vec![
            vec!['#', ' ', '#'],
            vec![' ', ' ', '#'],
            vec!['#', ' ', 'c'],
        ];
        assert!(place_word_in_crossword(board, "ca".into()));
    }
}
