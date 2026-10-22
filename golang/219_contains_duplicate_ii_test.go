package leetcode

import "testing"

func TestContainsNearbyDuplicateExampleOne(t *testing.T) {
	if !containsNearbyDuplicate([]int{1, 2, 3, 1}, 3) {
		t.Fatal("containsNearbyDuplicate() = false, want true")
	}
}

func TestContainsNearbyDuplicateExampleTwo(t *testing.T) {
	if !containsNearbyDuplicate([]int{1, 0, 1, 1}, 1) {
		t.Fatal("containsNearbyDuplicate() = false, want true")
	}
}

func TestContainsNearbyDuplicateExampleThree(t *testing.T) {
	if containsNearbyDuplicate([]int{1, 2, 3, 1, 2, 3}, 2) {
		t.Fatal("containsNearbyDuplicate() = true, want false")
	}
}
