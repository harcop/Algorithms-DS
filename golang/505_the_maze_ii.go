package leetcode

import "container/heap"

// LeetCode #505 - The Maze II
//
// The ball rolls until it hits a wall, and Dijkstra keeps the fewest steps among those stopping cells.
func shortestDistanceMaze(maze [][]int, start []int, destination []int) int {
	rows, cols := len(maze), len(maze[0])
	dist := make([][]int, rows)
	for r := range dist {
		dist[r] = make([]int, cols)
		for c := range dist[r] {
			dist[r][c] = int(^uint(0) >> 1)
		}
	}
	dist[start[0]][start[1]] = 0
	stops := &mazeStopHeap{}
	heap.Push(stops, mazeStop{0, start[0], start[1]})
	dirs := [][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
	for stops.Len() > 0 {
		cur := heap.Pop(stops).(mazeStop)
		if cur.dist > dist[cur.row][cur.col] {
			continue
		}
		if cur.row == destination[0] && cur.col == destination[1] {
			return cur.dist
		}
		for _, dir := range dirs {
			row, col, steps := cur.row, cur.col, 0
			for {
				nextRow, nextCol := row+dir[0], col+dir[1]
				if nextRow < 0 || nextCol < 0 || nextRow >= rows || nextCol >= cols || maze[nextRow][nextCol] != 0 {
					break
				}
				row, col = nextRow, nextCol
				steps++
			}
			nextDist := cur.dist + steps
			if nextDist < dist[row][col] {
				dist[row][col] = nextDist
				heap.Push(stops, mazeStop{nextDist, row, col})
			}
		}
	}
	return -1
}

type mazeStop struct {
	dist, row, col int
}

type mazeStopHeap []mazeStop

func (h mazeStopHeap) Len() int           { return len(h) }
func (h mazeStopHeap) Less(i, j int) bool { return h[i].dist < h[j].dist }
func (h mazeStopHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *mazeStopHeap) Push(x any) { *h = append(*h, x.(mazeStop)) }

func (h *mazeStopHeap) Pop() any {
	old := *h
	stop := old[len(old)-1]
	*h = old[:len(old)-1]
	return stop
}
