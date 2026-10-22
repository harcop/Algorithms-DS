package leetcode

import "testing"

func TestContainsNearbyAlmostDuplicateExampleOne(t *testing.T) {
	if !containsNearbyAlmostDuplicate([]int{1, 2, 3, 1}, 3, 0) {
		t.Fatal("containsNearbyAlmostDuplicate() = false, want true")
	}
}

func TestContainsNearbyAlmostDuplicateExampleTwo(t *testing.T) {
	if !containsNearbyAlmostDuplicate([]int{4, 1, 2, 3}, 2, 1) {
		t.Fatal("containsNearbyAlmostDuplicate() = false, want true")
	}
}
