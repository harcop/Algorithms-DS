package leetcode

import "testing"

func TestGeneratePossibleNextMovesExampleOne(t *testing.T) {
	got := generatePossibleNextMoves("++++")
	want := []string{"--++", "+--+", "++--"}
	if !sameStrings(got, want) {
		t.Fatalf("generatePossibleNextMoves() = %v, want %v", got, want)
	}
}
