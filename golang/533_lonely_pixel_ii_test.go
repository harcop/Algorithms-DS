package leetcode

import "testing"

func TestFindLonelyPixelIIExample(t *testing.T) {
	picture := [][]byte{
		{'W', 'B', 'W', 'B', 'B', 'W'},
		{'W', 'B', 'W', 'B', 'B', 'W'},
		{'W', 'B', 'W', 'B', 'B', 'W'},
		{'W', 'W', 'B', 'W', 'B', 'W'},
	}
	if got := findLonelyPixelII(picture, 3); got != 6 {
		t.Fatalf("findLonelyPixelII() = %d, want 6", got)
	}
}
