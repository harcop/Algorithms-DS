package leetcode

import (
	"sort"
	"testing"
)

func TestOuterTreesExamples(t *testing.T) {
	got := outerTrees([][]int{{1, 1}, {2, 2}, {2, 0}, {2, 4}, {3, 3}, {4, 2}})
	want := [][]int{{1, 1}, {2, 0}, {4, 2}, {3, 3}, {2, 4}}
	if !samePoints(got, want) {
		t.Fatalf("outerTrees = %v, want %v", got, want)
	}
	got = outerTrees([][]int{{1, 2}, {2, 2}, {4, 2}})
	want = [][]int{{1, 2}, {2, 2}, {4, 2}}
	if !samePoints(got, want) {
		t.Fatalf("outerTrees = %v, want %v", got, want)
	}
}

func samePoints(a, b [][]int) bool {
	if len(a) != len(b) {
		return false
	}
	key := func(points [][]int) [][2]int {
		out := make([][2]int, len(points))
		for i, point := range points {
			out[i] = [2]int{point[0], point[1]}
		}
		sort.Slice(out, func(i, j int) bool {
			if out[i][0] == out[j][0] {
				return out[i][1] < out[j][1]
			}
			return out[i][0] < out[j][0]
		})
		return out
	}
	left, right := key(a), key(b)
	for i := range left {
		if left[i] != right[i] {
			return false
		}
	}
	return true
}
