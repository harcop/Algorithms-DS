package leetcode

import "testing"

func TestSearchExampleOne(t *testing.T) {
	got := search([]int{4, 5, 6, 7, 0, 1, 2}, 0)
	if got != 4 {
		t.Fatalf("search() = %d, want 4", got)
	}
}

func TestSearchExampleTwo(t *testing.T) {
	got := search([]int{4, 5, 6, 7, 0, 1, 2}, 3)
	if got != -1 {
		t.Fatalf("search() = %d, want -1", got)
	}
}

func TestSearchExampleThree(t *testing.T) {
	got := search([]int{1}, 0)
	if got != -1 {
		t.Fatalf("search() = %d, want -1", got)
	}
}
