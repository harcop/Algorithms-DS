package leetcode

import "testing"

func TestCanFinishExampleOne(t *testing.T) {
	if !canFinish(2, [][]int{{1, 0}}) {
		t.Fatal("canFinish() = false, want true")
	}
}

func TestCanFinishExampleTwo(t *testing.T) {
	if canFinish(2, [][]int{{1, 0}, {0, 1}}) {
		t.Fatal("canFinish() = true, want false")
	}
}
