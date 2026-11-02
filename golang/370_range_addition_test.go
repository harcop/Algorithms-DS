package leetcode

import "testing"

func TestGetModifiedArrayExample(t *testing.T) {
	updates := [][]int{{1, 3, 2}, {2, 4, 3}, {0, 2, -2}}
	got := getModifiedArray(5, updates)
	want := []int{-2, 0, 3, 5, 3}
	if !sameInts(got, want) {
		t.Fatalf("getModifiedArray() = %v, want %v", got, want)
	}
}
