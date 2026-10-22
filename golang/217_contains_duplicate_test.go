package leetcode

import "testing"

func TestContainsDuplicateExampleOne(t *testing.T) {
	if !containsDuplicate([]int{1, 2, 3, 1}) {
		t.Fatal("containsDuplicate() = false, want true")
	}
}

func TestContainsDuplicateExampleTwo(t *testing.T) {
	if containsDuplicate([]int{1, 2, 3, 4}) {
		t.Fatal("containsDuplicate() = true, want false")
	}
}

func TestContainsDuplicateExampleThree(t *testing.T) {
	if !containsDuplicate([]int{1, 1, 1, 3, 3, 4, 3, 2, 4, 2}) {
		t.Fatal("containsDuplicate() = false, want true")
	}
}
