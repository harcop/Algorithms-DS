package leetcode

import "testing"

func TestReconstructQueueExample(t *testing.T) {
	got := reconstructQueue([][]int{{7, 0}, {4, 4}, {7, 1}, {5, 0}, {6, 1}, {5, 2}})
	want := [][]int{{5, 0}, {7, 0}, {5, 2}, {6, 1}, {4, 4}, {7, 1}}
	if len(got) != len(want) {
		t.Fatalf("reconstructQueue() = %v, want %v", got, want)
	}
	for i := range want {
		if !sameInts(got[i], want[i]) {
			t.Fatalf("reconstructQueue() = %v, want %v", got, want)
		}
	}
}
