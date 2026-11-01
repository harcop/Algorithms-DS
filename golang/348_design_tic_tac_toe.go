package leetcode

// LeetCode #348 - Design Tic-Tac-Toe
//
// One counter per row, column, and diagonal records a win as soon as a line fills.
type TicTacToe struct {
	n    int
	row  []int
	col  []int
	diag int
	anti int
}

func NewTicTacToe(n int) *TicTacToe {
	return &TicTacToe{
		n:   n,
		row: make([]int, n),
		col: make([]int, n),
	}
}

func (g *TicTacToe) Move(row, col, player int) int {
	mark := 1
	if player != 1 {
		mark = -1
	}
	g.row[row] += mark
	g.col[col] += mark
	if row == col {
		g.diag += mark
	}
	if row+col == g.n-1 {
		g.anti += mark
	}
	if absInt(g.row[row]) == g.n || absInt(g.col[col]) == g.n || absInt(g.diag) == g.n || absInt(g.anti) == g.n {
		return player
	}
	return 0
}
