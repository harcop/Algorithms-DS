package leetcode

import "testing"

func TestFindLonelyPixelExample(t *testing.T) {
	picture := [][]byte{
		{'W', 'W', 'B'},
		{'W', 'B', 'W'},
		{'B', 'W', 'W'},
	}
	if got := findLonelyPixel(picture); got != 3 {
		t.Fatalf("findLonelyPixel() = %d, want 3", got)
	}
}
