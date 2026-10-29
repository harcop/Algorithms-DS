package leetcode

import "testing"

func TestFindMinHeightTreesExampleOne(t *testing.T) {
	got := findMinHeightTrees(4, [][]int{{1, 0}, {1, 2}, {1, 3}})
	want := []int{1}
	if !sameInts(got, want) {
		t.Fatalf("findMinHeightTrees() = %v, want %v", got, want)
	}
}

func TestFindMinHeightTreesExampleTwo(t *testing.T) {
	got := findMinHeightTrees(6, [][]int{{3, 0}, {3, 1}, {3, 2}, {3, 4}, {5, 4}})
	want := []int{3, 4}
	if !sameInts(got, want) {
		t.Fatalf("findMinHeightTrees() = %v, want %v", got, want)
	}
}
