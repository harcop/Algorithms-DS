package leetcode

import "testing"

func TestInsertExampleOne(t *testing.T) {
	got := insert([][]int{{1, 3}, {6, 9}}, []int{2, 5})
	want := [][]int{{1, 5}, {6, 9}}
	if !sameMatrix(got, want) {
		t.Fatalf("insert() = %v, want %v", got, want)
	}
}

func TestInsertExampleTwo(t *testing.T) {
	got := insert([][]int{{1, 2}, {3, 5}, {6, 7}, {8, 10}, {12, 16}}, []int{4, 8})
	want := [][]int{{1, 2}, {3, 10}, {12, 16}}
	if !sameMatrix(got, want) {
		t.Fatalf("insert() = %v, want %v", got, want)
	}
}
