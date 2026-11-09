package leetcode

import "testing"

func TestFindMaxFormExamples(t *testing.T) {
	got := findMaxForm([]string{"10", "0001", "111001", "1", "0"}, 5, 3)
	if got != 4 {
		t.Fatalf("findMaxForm() = %d, want 4", got)
	}
	got = findMaxForm([]string{"10", "0", "1"}, 1, 1)
	if got != 2 {
		t.Fatalf("findMaxForm() = %d, want 2", got)
	}
}
