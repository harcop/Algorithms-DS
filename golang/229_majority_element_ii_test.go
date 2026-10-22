package leetcode

import "testing"

func TestMajorityElementIIExampleOne(t *testing.T) {
	got := majorityElementII([]int{3, 2, 3})
	want := []int{3}
	if !sameInts(got, want) {
		t.Fatalf("majorityElementII() = %v, want %v", got, want)
	}
}

func TestMajorityElementIIExampleTwo(t *testing.T) {
	got := majorityElementII([]int{1})
	want := []int{1}
	if !sameInts(got, want) {
		t.Fatalf("majorityElementII() = %v, want %v", got, want)
	}
}
