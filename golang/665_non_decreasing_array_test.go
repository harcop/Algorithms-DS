package leetcode

import "testing"

func TestCheckPossibilityExamples(t *testing.T) {
	if !checkPossibility([]int{4, 2, 3}) {
		t.Fatal("checkPossibility([4 2 3]) = false, want true")
	}
	if checkPossibility([]int{4, 2, 1}) {
		t.Fatal("checkPossibility([4 2 1]) = true, want false")
	}
}
