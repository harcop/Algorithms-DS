package leetcode

import "testing"

func TestMaxSlidingWindowExampleOne(t *testing.T) {
	got := maxSlidingWindow([]int{1, 3, -1, -3, 5, 3, 6, 7}, 3)
	want := []int{3, 3, 5, 5, 6, 7}
	if !sameInts(got, want) {
		t.Fatalf("maxSlidingWindow() = %v, want %v", got, want)
	}
}

func TestMaxSlidingWindowExampleTwo(t *testing.T) {
	got := maxSlidingWindow([]int{1}, 1)
	want := []int{1}
	if !sameInts(got, want) {
		t.Fatalf("maxSlidingWindow() = %v, want %v", got, want)
	}
}
