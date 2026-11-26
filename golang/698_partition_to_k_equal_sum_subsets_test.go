package leetcode

import "testing"

func TestCanPartitionKSubsetsExamples(t *testing.T) {
	if !canPartitionKSubsets([]int{4, 3, 2, 3, 5, 2, 1}, 4) {
		t.Fatal("canPartitionKSubsets() = false, want true")
	}
	if canPartitionKSubsets([]int{1, 2, 3, 4}, 3) {
		t.Fatal("canPartitionKSubsets() = true, want false")
	}
}
