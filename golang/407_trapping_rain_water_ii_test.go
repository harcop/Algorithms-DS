package leetcode

import "testing"

func TestTrapRainWaterExamples(t *testing.T) {
	got := trapRainWater([][]int{
		{1, 4, 3, 1, 3, 2},
		{3, 2, 1, 3, 2, 4},
		{2, 3, 3, 2, 3, 1},
	})
	if got != 4 {
		t.Fatalf("trapRainWater() = %d, want 4", got)
	}
	got = trapRainWater([][]int{
		{3, 3, 3, 3, 3},
		{3, 2, 2, 2, 3},
		{3, 2, 1, 2, 3},
		{3, 2, 2, 2, 3},
		{3, 3, 3, 3, 3},
	})
	if got != 10 {
		t.Fatalf("trapRainWater() = %d, want 10", got)
	}
}
