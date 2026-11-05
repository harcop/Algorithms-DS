package leetcode

import (
	"sort"
	"testing"
)

func TestPacificAtlanticExample(t *testing.T) {
	got := pacificAtlantic([][]int{
		{1, 2, 2, 3, 5},
		{3, 2, 3, 4, 4},
		{2, 4, 5, 3, 1},
		{6, 7, 1, 4, 5},
		{5, 1, 1, 2, 4},
	})
	want := [][]int{{0, 4}, {1, 3}, {1, 4}, {2, 2}, {3, 0}, {3, 1}, {4, 0}}
	if !sameCells(got, want) {
		t.Fatalf("pacificAtlantic() = %v, want %v", got, want)
	}
}

func sameCells(a, b [][]int) bool {
	if len(a) != len(b) {
		return false
	}
	order := func(cells [][]int) {
		sort.Slice(cells, func(i, j int) bool {
			if cells[i][0] != cells[j][0] {
				return cells[i][0] < cells[j][0]
			}
			return cells[i][1] < cells[j][1]
		})
	}
	order(a)
	order(b)
	for i := range a {
		if !sameInts(a[i], b[i]) {
			return false
		}
	}
	return true
}
