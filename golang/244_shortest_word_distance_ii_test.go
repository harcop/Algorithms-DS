package leetcode

import "testing"

func TestWordDistanceExample(t *testing.T) {
	words := NewWordDistance([]string{"practice", "makes", "perfect", "coding", "makes"})
	if got := words.Shortest("makes", "coding"); got != 1 {
		t.Fatalf("Shortest() = %d, want 1", got)
	}
	if got := words.Shortest("makes", "makes"); got != 3 {
		t.Fatalf("Shortest() = %d, want 3", got)
	}
}
