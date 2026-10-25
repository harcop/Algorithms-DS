package leetcode

import "testing"

func TestShortestWordDistanceExampleOne(t *testing.T) {
	words := []string{"practice", "makes", "perfect", "coding", "makes"}
	if got := shortestWordDistance(words, "makes", "coding"); got != 1 {
		t.Fatalf("shortestWordDistance() = %d, want 1", got)
	}
}

func TestShortestWordDistanceExampleTwo(t *testing.T) {
	words := []string{"practice", "makes", "perfect", "coding", "makes"}
	if got := shortestWordDistance(words, "makes", "makes"); got != 3 {
		t.Fatalf("shortestWordDistance() = %d, want 3", got)
	}
}
