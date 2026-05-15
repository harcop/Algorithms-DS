/// LeetCode #794 - Valid Tic-Tac-Toe State
fn valid_tic_tac_toe(board: Vec<String>) -> bool {
    let mut x = 0i32;
    let mut o = 0i32;
    let b: Vec<Vec<u8>> = board.iter().map(|r| r.bytes().collect()).collect();
    for r in &b {
        for &c in r {
            if c == b'X' {
                x += 1;
            } else if c == b'O' {
                o += 1;
            }
        }
    }
    if o > x + 1 || x > o + 1 {
        return false;
    }
    fn win(b: &Vec<Vec<u8>>, p: u8) -> bool {
        for i in 0..3 {
            if b[i][0] == p && b[i][1] == p && b[i][2] == p {
                return true;
            }
            if b[0][i] == p && b[1][i] == p && b[2][i] == p {
                return true;
            }
        }
        b[0][0] == p && b[1][1] == p && b[2][2] == p || b[0][2] == p && b[1][1] == p && b[2][0] == p
    }
    let xw = win(&b, b'X');
    let ow = win(&b, b'O');
    if xw && o > 0 {
        return false;
    }
    if ow && x != o {
        return false;
    }
    if xw && x == o {
        return false;
    }
    if ow && x != o + 1 {
        return false;
    }
    true
}

fn main() {
    let b = vec!["O  ".into(), "   ".into(), "   ".into()];
    println!("{}", valid_tic_tac_toe(b));
}

#[cfg(test)]
mod tests {
    use super::valid_tic_tac_toe;

    #[test]
    fn example_one() {
        assert!(valid_tic_tac_toe(vec!["O  ".into(), "   ".into(), "   ".into()]));
    }

    #[test]
    fn example_two() {
        assert!(!valid_tic_tac_toe(vec!["XOX".into(), " X ".into(), "   ".into()]));
    }
}
