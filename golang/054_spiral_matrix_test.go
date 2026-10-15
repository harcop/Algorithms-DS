package leetcode

import "testing"

func TestSpiralOrderExampleOne(t *testing.T) {
	got := spiralOrder([][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}})
	want := []int{1, 2, 3, 6, 9, 8, 7, 4, 5}
	if !sameInts(got, want) {
		t.Fatalf("spiralOrder() = %v, want %v", got, want)
	}
}

func TestSpiralOrderExampleTwo(t *testing.T) {
	got := spiralOrder([][]int{{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}})
	want := []int{1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7}
	if !sameInts(got, want) {
		t.Fatalf("spiralOrder() = %v, want %v", got, want)
	}
}
