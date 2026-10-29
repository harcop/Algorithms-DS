package leetcode

import "testing"

func TestNumIslands2ExampleOne(t *testing.T) {
	got := numIslands2(3, 3, [][]int{{0, 0}, {0, 1}, {1, 2}, {2, 1}})
	want := []int{1, 1, 2, 3}
	if !sameInts(got, want) {
		t.Fatalf("numIslands2() = %v, want %v", got, want)
	}
}
