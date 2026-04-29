/// LeetCode #79 - Word Search
fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
    let w = word.as_bytes();
    let m = board.len();
    let n = board[0].len();

    fn dfs(
        board: &mut Vec<Vec<char>>,
        word: &[u8],
        i: usize,
        j: usize,
        k: usize,
    ) -> bool {
        if k == word.len() {
            return true;
        }
        if i >= board.len() || j >= board[0].len() {
            return false;
        }
        if board[i][j] != word[k] as char {
            return false;
        }

        let orig = board[i][j];
        board[i][j] = '#';

        let ok = (i > 0 && dfs(board, word, i - 1, j, k + 1))
            || dfs(board, word, i + 1, j, k + 1)
            || (j > 0 && dfs(board, word, i, j - 1, k + 1))
            || dfs(board, word, i, j + 1, k + 1);

        board[i][j] = orig;
        ok
    }

    for i in 0..m {
        for j in 0..n {
            if dfs(&mut board, w, i, j, 0) {
                return true;
            }
        }
    }
    false
}

fn main() {
    println!(
        "{}",
        exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "ABCCED".to_string(),
        )
    );
}

#[cfg(test)]
mod tests {
    use super::exist;

    #[test]
    fn example_one() {
        assert!(exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "ABCCED".to_string(),
        ));
    }

    #[test]
    fn example_two() {
        assert!(exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "SEE".to_string(),
        ));
    }

    #[test]
    fn example_three() {
        assert!(!exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "ABCB".to_string(),
        ));
    }
}
