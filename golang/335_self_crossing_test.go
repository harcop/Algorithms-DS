package leetcode

import "testing"

func TestIsSelfCrossingExamples(t *testing.T) {
	if !isSelfCrossing([]int{2, 1, 1, 2}) {
		t.Fatal("isSelfCrossing() = false, want true")
	}
	if isSelfCrossing([]int{1, 2, 3, 4}) {
		t.Fatal("isSelfCrossing() = true, want false")
	}
	if !isSelfCrossing([]int{1, 1, 1, 1}) {
		t.Fatal("isSelfCrossing() = false, want true")
	}
}
