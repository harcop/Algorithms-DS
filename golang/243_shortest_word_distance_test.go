package leetcode

import "testing"

func TestShortestDistanceExampleOne(t *testing.T) {
	words := []string{"practice", "makes", "perfect", "coding", "makes"}
	if got := shortestDistance(words, "makes", "coding"); got != 1 {
		t.Fatalf("shortestDistance() = %d, want 1", got)
	}
}

func TestShortestDistanceExampleTwo(t *testing.T) {
	words := []string{"practice", "makes", "perfect", "coding", "makes"}
	if got := shortestDistance(words, "makes", "makes"); got != 3 {
		t.Fatalf("shortestDistance() = %d, want 3", got)
	}
}
