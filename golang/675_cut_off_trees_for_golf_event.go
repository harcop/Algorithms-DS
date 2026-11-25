package leetcode

import "sort"

// LeetCode #675 - Cut Off Trees for Golf Event
//
// Trees are cut from shortest to tallest, and each leg is the shortest walk that avoids obstacles.
func cutOffTree(forest [][]int) int {
	type tree struct{ height, row, col int }
	var trees []tree
	for r := range forest {
		for c, height := range forest[r] {
			if height > 1 {
				trees = append(trees, tree{height, r, c})
			}
		}
	}
	sort.Slice(trees, func(i, j int) bool {
		return trees[i].height < trees[j].height
	})
	steps, row, col := 0, 0, 0
	for _, next := range trees {
		dist := forestSteps(forest, row, col, next.row, next.col)
		if dist < 0 {
			return -1
		}
		steps += dist
		row, col = next.row, next.col
	}
	return steps
}

func forestSteps(forest [][]int, sr, sc, tr, tc int) int {
	if sr == tr && sc == tc {
		return 0
	}
	rows, cols := len(forest), len(forest[0])
	seen := make([][]bool, rows)
	for r := range seen {
		seen[r] = make([]bool, cols)
	}
	type step struct{ row, col, dist int }
	queue := []step{{sr, sc, 0}}
	seen[sr][sc] = true
	dirs := [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	for head := 0; head < len(queue); head++ {
		cur := queue[head]
		for _, dir := range dirs {
			nr, nc := cur.row+dir[0], cur.col+dir[1]
			if nr < 0 || nc < 0 || nr >= rows || nc >= cols || seen[nr][nc] || forest[nr][nc] == 0 {
				continue
			}
			if nr == tr && nc == tc {
				return cur.dist + 1
			}
			seen[nr][nc] = true
			queue = append(queue, step{nr, nc, cur.dist + 1})
		}
	}
	return -1
}
