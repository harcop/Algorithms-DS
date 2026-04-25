use std::collections::HashSet;

/// LeetCode #36 - Valid Sudoku
fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for row in 0..9 {
        let mut seen = HashSet::new();
        for col in 0..9 {
            let c = board[row][col];
            if c == '.' {
                continue;
            }
            if !seen.insert(c) {
                return false;
            }
        }
    }

    for col in 0..9 {
        let mut seen = HashSet::new();
        for row in 0..9 {
            let c = board[row][col];
            if c == '.' {
                continue;
            }
            if !seen.insert(c) {
                return false;
            }
        }
    }

    for block in 0..9 {
        let br = 3 * (block / 3);
        let bc = 3 * (block % 3);
        let mut seen = HashSet::new();
        for r in 0..3 {
            for c in 0..3 {
                let ch = board[br + r][bc + c];
                if ch == '.' {
                    continue;
                }
                if !seen.insert(ch) {
                    return false;
                }
            }
        }
    }

    true
}

fn main() {
    let board: Vec<Vec<char>> = [
        "53..7....", "6..195...", ".98....6.", "8...6...3", "4..8.3..1", "7...2...6", ".6....28.",
        "...419..5", "....8..79",
    ]
    .iter()
    .map(|r| r.chars().collect())
    .collect();
    println!("{}", is_valid_sudoku(board));
}

#[cfg(test)]
mod tests {
    use super::is_valid_sudoku;

    fn board_from(s: [&str; 9]) -> Vec<Vec<char>> {
        s.iter().map(|r| r.chars().collect()).collect()
    }

    #[test]
    fn example_one() {
        let b = board_from([
            "53..7....", "6..195...", ".98....6.", "8...6...3", "4..8.3..1", "7...2...6",
            ".6....28.", "...419..5", "....8..79",
        ]);
        assert!(is_valid_sudoku(b));
    }

    #[test]
    fn example_two() {
        let b = board_from([
            "83..7....", "6..195...", ".98....6.", "8...6...3", "4..8.3..1", "7...2...6",
            ".6....28.", "...419..5", "....8..79",
        ]);
        assert!(!is_valid_sudoku(b));
    }
}
