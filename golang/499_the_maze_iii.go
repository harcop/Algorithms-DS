package leetcode

import "container/heap"

// LeetCode #499 - The Maze III
//
// Roll until a wall or the hole, and keep the shortest route, breaking ties by the lexicographically smaller direction string.
func findShortestWay(maze [][]int, ball []int, hole []int) string {
	rows, cols := len(maze), len(maze[0])
	dist := make([][]int, rows)
	bestPath := make([][]string, rows)
	for r := range dist {
		dist[r] = make([]int, cols)
		bestPath[r] = make([]string, cols)
		for c := range dist[r] {
			dist[r][c] = int(^uint(0) >> 1)
			bestPath[r][c] = "~"
		}
	}
	startRow, startCol := ball[0], ball[1]
	dist[startRow][startCol] = 0
	bestPath[startRow][startCol] = ""
	directions := []struct {
		row, col int
		step     byte
	}{{1, 0, 'd'}, {0, -1, 'l'}, {0, 1, 'r'}, {-1, 0, 'u'}}
	routes := &mazeRouteHeap{}
	heap.Push(routes, mazeRoute{0, "", startRow, startCol})
	for routes.Len() > 0 {
		route := heap.Pop(routes).(mazeRoute)
		if route.row == hole[0] && route.col == hole[1] {
			return route.path
		}
		if route.dist > dist[route.row][route.col] || (route.dist == dist[route.row][route.col] && route.path > bestPath[route.row][route.col]) {
			continue
		}
		for _, dir := range directions {
			row, col := route.row, route.col
			steps := 0
			for {
				nextRow, nextCol := row+dir.row, col+dir.col
				if nextRow < 0 || nextCol < 0 || nextRow >= rows || nextCol >= cols || maze[nextRow][nextCol] != 0 {
					break
				}
				row, col = nextRow, nextCol
				steps++
				if row == hole[0] && col == hole[1] {
					break
				}
			}
			if steps == 0 {
				continue
			}
			nextDist := route.dist + steps
			nextPath := route.path + string(dir.step)
			if nextDist < dist[row][col] || (nextDist == dist[row][col] && nextPath < bestPath[row][col]) {
				dist[row][col] = nextDist
				bestPath[row][col] = nextPath
				heap.Push(routes, mazeRoute{nextDist, nextPath, row, col})
			}
		}
	}
	return "impossible"
}

type mazeRoute struct {
	dist int
	path string
	row  int
	col  int
}

type mazeRouteHeap []mazeRoute

func (h mazeRouteHeap) Len() int { return len(h) }
func (h mazeRouteHeap) Less(i, j int) bool {
	if h[i].dist != h[j].dist {
		return h[i].dist < h[j].dist
	}
	return h[i].path < h[j].path
}
func (h mazeRouteHeap) Swap(i, j int) { h[i], h[j] = h[j], h[i] }

func (h *mazeRouteHeap) Push(x any) { *h = append(*h, x.(mazeRoute)) }

func (h *mazeRouteHeap) Pop() any {
	old := *h
	item := old[len(old)-1]
	*h = old[:len(old)-1]
	return item
}
