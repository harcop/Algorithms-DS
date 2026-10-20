package leetcode

import "testing"

func TestCalculateMinimumHPExampleOne(t *testing.T) {
	dungeon := [][]int{{-2, -3, 3}, {-5, -10, 1}, {10, 30, -5}}
	if got := calculateMinimumHP(dungeon); got != 7 {
		t.Fatalf("calculateMinimumHP() = %d, want 7", got)
	}
}

func TestCalculateMinimumHPExampleTwo(t *testing.T) {
	if got := calculateMinimumHP([][]int{{0}}); got != 1 {
		t.Fatalf("calculateMinimumHP() = %d, want 1", got)
	}
}
