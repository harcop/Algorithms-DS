package leetcode

import "testing"

func TestGetMaxRepetitionsExample(t *testing.T) {
	if got := getMaxRepetitions("acb", 4, "ab", 2); got != 2 {
		t.Fatalf("getMaxRepetitions() = %d, want 2", got)
	}
}
