package leetcode

import "testing"

func TestCanPartitionExamples(t *testing.T) {
	if !canPartition([]int{1, 5, 11, 5}) {
		t.Fatal("canPartition([1,5,11,5]) = false, want true")
	}
	if canPartition([]int{1, 2, 3, 5}) {
		t.Fatal("canPartition([1,2,3,5]) = true, want false")
	}
}
