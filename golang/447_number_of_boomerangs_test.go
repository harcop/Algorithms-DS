package leetcode

import "testing"

func TestNumberOfBoomerangsExample(t *testing.T) {
	got := numberOfBoomerangs([][]int{{0, 0}, {1, 0}, {2, 0}})
	if got != 2 {
		t.Fatalf("numberOfBoomerangs() = %d, want 2", got)
	}
}
