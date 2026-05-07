/// LeetCode #348 - Design Tic-Tac-Toe (n×n counters per player)
struct TicTacToe {
    n: usize,
    row: Vec<i32>,
    col: Vec<i32>,
    diag: i32,
    anti: i32,
}

impl TicTacToe {
    fn new(n: i32) -> Self {
        let n = n as usize;
        TicTacToe {
            n,
            row: vec![0; n],
            col: vec![0; n],
            diag: 0,
            anti: 0,
        }
    }

    /// player is 1 or 2
    fn make_move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        let p = if player == 1 { 1 } else { -1 };
        let r = row as usize;
        let c = col as usize;
        self.row[r] += p;
        self.col[c] += p;
        if r == c {
            self.diag += p;
        }
        if r + c == self.n - 1 {
            self.anti += p;
        }
        let win = self.row[r].abs() as usize == self.n
            || self.col[c].abs() as usize == self.n
            || self.diag.abs() as usize == self.n
            || self.anti.abs() as usize == self.n;
        if win { player } else { 0 }
    }
}

fn main() {
    let mut g = TicTacToe::new(3);
    assert_eq!(g.make_move(0, 0, 1), 0);
}

#[cfg(test)]
mod tests {
    use super::TicTacToe;

    #[test]
    fn column_win_player1() {
        let mut g = TicTacToe::new(3);
        assert_eq!(g.make_move(0, 0, 1), 0);
        assert_eq!(g.make_move(0, 1, 2), 0);
        assert_eq!(g.make_move(1, 0, 1), 0);
        assert_eq!(g.make_move(0, 2, 2), 0);
        assert_eq!(g.make_move(2, 0, 1), 1);
    }
}
