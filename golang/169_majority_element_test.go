package leetcode

import "testing"

func TestMajorityElementExampleOne(t *testing.T) {
	got := majorityElement([]int{3, 2, 3})
	if got != 3 {
		t.Fatalf("majorityElement() = %d, want 3", got)
	}
}

func TestMajorityElementExampleTwo(t *testing.T) {
	got := majorityElement([]int{2, 2, 1, 1, 1, 2, 2})
	if got != 2 {
		t.Fatalf("majorityElement() = %d, want 2", got)
	}
}
