package leetcode

import "testing"

func TestFindCircleNumExample(t *testing.T) {
	got := findCircleNum([][]int{{1, 1, 0}, {1, 1, 0}, {0, 0, 1}})
	if got != 2 {
		t.Fatalf("findCircleNum = %d, want 2", got)
	}
}
