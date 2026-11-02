package leetcode

// LeetCode #353 - Design Snake Game
//
// The tail steps off before the head moves, so the snake can follow its own tail.
type SnakeGame struct {
	food [][]int
	next int
	rows int
	cols int
	body [][2]int
	used map[[2]int]struct{}
}

func NewSnakeGame(width, height int, food [][]int) *SnakeGame {
	start := [2]int{0, 0}
	return &SnakeGame{
		food: food,
		rows: height,
		cols: width,
		body: [][2]int{start},
		used: map[[2]int]struct{}{start: {}},
	}
}

func (g *SnakeGame) Move(direction string) int {
	head := g.body[len(g.body)-1]
	row, col := head[0], head[1]
	switch direction {
	case "U":
		row--
	case "D":
		row++
	case "L":
		col--
	default:
		col++
	}
	if row < 0 || col < 0 || row >= g.rows || col >= g.cols {
		return -1
	}
	eating := g.next < len(g.food) && g.food[g.next][0] == row && g.food[g.next][1] == col
	if !eating {
		tail := g.body[0]
		g.body = g.body[1:]
		delete(g.used, tail)
	}
	cell := [2]int{row, col}
	if _, occupied := g.used[cell]; occupied {
		return -1
	}
	g.body = append(g.body, cell)
	g.used[cell] = struct{}{}
	if eating {
		g.next++
	}
	return len(g.body) - 1
}
