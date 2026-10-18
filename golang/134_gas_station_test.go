package leetcode

import "testing"

func TestCanCompleteCircuitExampleOne(t *testing.T) {
	got := canCompleteCircuit([]int{1, 2, 3, 4, 5}, []int{3, 4, 5, 1, 2})
	if got != 3 {
		t.Fatalf("canCompleteCircuit() = %d, want 3", got)
	}
}

func TestCanCompleteCircuitExampleTwo(t *testing.T) {
	got := canCompleteCircuit([]int{2, 3, 4}, []int{3, 4, 3})
	if got != -1 {
		t.Fatalf("canCompleteCircuit() = %d, want -1", got)
	}
}
