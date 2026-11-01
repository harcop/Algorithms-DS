package leetcode

import "testing"

func TestCountComponentsExample(t *testing.T) {
	edges := [][]int{{0, 1}, {1, 2}, {3, 4}}
	if got := countComponents(5, edges); got != 2 {
		t.Fatalf("countComponents() = %d, want 2", got)
	}
}
