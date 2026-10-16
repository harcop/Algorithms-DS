package leetcode

import "testing"

func TestPartitionExampleOne(t *testing.T) {
	got := listToVec(partition(vecToList([]int{1, 4, 3, 2, 5, 2}), 3))
	want := []int{1, 2, 2, 4, 3, 5}
	if !sameInts(got, want) {
		t.Fatalf("partition() = %v, want %v", got, want)
	}
}

func TestPartitionExampleTwo(t *testing.T) {
	got := listToVec(partition(vecToList([]int{2, 1}), 2))
	want := []int{1, 2}
	if !sameInts(got, want) {
		t.Fatalf("partition() = %v, want %v", got, want)
	}
}
