package leetcode

import "testing"

func TestFindOrderExampleOne(t *testing.T) {
	got := findOrder(2, [][]int{{1, 0}})
	want := []int{0, 1}
	if !sameInts(got, want) {
		t.Fatalf("findOrder() = %v, want %v", got, want)
	}
}

func TestFindOrderExampleTwo(t *testing.T) {
	got := findOrder(1, nil)
	want := []int{0}
	if !sameInts(got, want) {
		t.Fatalf("findOrder() = %v, want %v", got, want)
	}
}

func TestFindOrderExampleThree(t *testing.T) {
	got := findOrder(2, [][]int{{1, 0}, {0, 1}})
	if len(got) != 0 {
		t.Fatalf("findOrder() = %v, want none", got)
	}
}
