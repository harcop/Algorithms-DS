package leetcode

import "testing"

func TestMinSquirrelDistanceExample(t *testing.T) {
	got := minSquirrelDistance(5, 7, []int{2, 2}, []int{4, 4}, [][]int{{3, 0}, {2, 5}})
	if got != 12 {
		t.Fatalf("minSquirrelDistance = %d, want 12", got)
	}
}
