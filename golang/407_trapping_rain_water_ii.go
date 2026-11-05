package leetcode

import "container/heap"

// LeetCode #407 - Trapping Rain Water II
//
// Expand inward from the shortest border cell, and each neighbor is held back by the tallest wall seen on the way in.
func trapRainWater(heightMap [][]int) int {
	rows := len(heightMap)
	if rows == 0 || len(heightMap[0]) == 0 {
		return 0
	}
	cols := len(heightMap[0])
	seen := make([][]bool, rows)
	for r := range seen {
		seen[r] = make([]bool, cols)
	}
	border := &rainBorderHeap{}
	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if r == 0 || c == 0 || r == rows-1 || c == cols-1 {
				heap.Push(border, rainCell{heightMap[r][c], r, c})
				seen[r][c] = true
			}
		}
	}
	water := 0
	dirs := [][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
	for border.Len() > 0 {
		cell := heap.Pop(border).(rainCell)
		for _, dir := range dirs {
			nr, nc := cell.r+dir[0], cell.c+dir[1]
			if nr < 0 || nc < 0 || nr >= rows || nc >= cols || seen[nr][nc] {
				continue
			}
			seen[nr][nc] = true
			height := heightMap[nr][nc]
			if cell.h > height {
				water += cell.h - height
			}
			next := cell.h
			if height > next {
				next = height
			}
			heap.Push(border, rainCell{next, nr, nc})
		}
	}
	return water
}

type rainCell struct{ h, r, c int }

type rainBorderHeap []rainCell

func (h rainBorderHeap) Len() int           { return len(h) }
func (h rainBorderHeap) Less(i, j int) bool { return h[i].h < h[j].h }
func (h rainBorderHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *rainBorderHeap) Push(x any) { *h = append(*h, x.(rainCell)) }

func (h *rainBorderHeap) Pop() any {
	old := *h
	item := old[len(old)-1]
	*h = old[:len(old)-1]
	return item
}
