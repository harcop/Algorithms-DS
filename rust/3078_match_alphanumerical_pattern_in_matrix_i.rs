/// LeetCode #3078 - Match Alphanumerical Pattern in Matrix I
fn find_pattern(board: Vec<Vec<i32>>, pattern: Vec<String>) -> Vec<i32> {
    let m = board.len();
    let n = board[0].len();
    let r = pattern.len();
    let c = pattern[0].len();

    for i in 0..=m - r {
        for j in 0..=n - c {
            if matches(&board, &pattern, i, j) {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![-1, -1]
}

fn matches(board: &[Vec<i32>], pattern: &[String], si: usize, sj: usize) -> bool {
    let mut letter_to_digit = [-1i32; 26];
    let mut digit_to_letter = [-1i32; 10];

    for (di, row) in pattern.iter().enumerate() {
        for (dj, ch) in row.chars().enumerate() {
            let cell = board[si + di][sj + dj];
            if ch.is_ascii_digit() {
                if cell != (ch as u8 - b'0') as i32 {
                    return false;
                }
            } else {
                let li = (ch as u8 - b'a') as usize;
                let d = cell as usize;
                if letter_to_digit[li] == -1 && digit_to_letter[d] == -1 {
                    letter_to_digit[li] = cell;
                    digit_to_letter[d] = li as i32;
                } else if letter_to_digit[li] != cell || digit_to_letter[d] != li as i32 {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let board = vec![vec![1, 2, 2], vec![2, 2, 3], vec![2, 3, 3]];
    let pattern = vec!["ab".into(), "bb".into()];
    println!("{:?}", find_pattern(board, pattern));
}

#[cfg(test)]
mod tests {
    use super::find_pattern;

    #[test]
    fn example1() {
        let board = vec![vec![1, 2, 2], vec![2, 2, 3], vec![2, 3, 3]];
        let pattern = vec!["ab".into(), "bb".into()];
        assert_eq!(find_pattern(board, pattern), vec![0, 0]);
    }

    #[test]
    fn example2() {
        let board = vec![vec![1, 1, 2], vec![3, 3, 4], vec![6, 6, 6]];
        let pattern = vec!["ab".into(), "66".into()];
        assert_eq!(find_pattern(board, pattern), vec![1, 1]);
    }

    #[test]
    fn example3() {
        let board = vec![vec![1, 2], vec![2, 1]];
        let pattern = vec!["xx".into()];
        assert_eq!(find_pattern(board, pattern), vec![-1, -1]);
    }
}
