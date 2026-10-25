package leetcode

import "testing"

func TestAlienOrderExampleOne(t *testing.T) {
	words := []string{"wrt", "wrf", "er", "ett", "rftt"}
	if got := alienOrder(words); len(got) != 5 {
		t.Fatalf("alienOrder() = %q, want 5 letters", got)
	}
}
