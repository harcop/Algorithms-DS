package leetcode

import "testing"

func TestMaxKilledEnemiesExample(t *testing.T) {
	grid := [][]byte{
		{'0', 'E', '0', '0'},
		{'E', '0', 'W', 'E'},
		{'0', 'E', '0', '0'},
	}
	if got := maxKilledEnemies(grid); got != 3 {
		t.Fatalf("maxKilledEnemies() = %d, want 3", got)
	}
}
