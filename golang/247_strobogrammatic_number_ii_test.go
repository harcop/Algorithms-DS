package leetcode

import "testing"

func TestFindStrobogrammaticExampleOne(t *testing.T) {
	got := findStrobogrammatic(2)
	want := []string{"11", "69", "88", "96"}
	if !sameStrings(got, want) {
		t.Fatalf("findStrobogrammatic() = %v, want %v", got, want)
	}
}

func TestFindStrobogrammaticExampleTwo(t *testing.T) {
	got := findStrobogrammatic(1)
	want := []string{"0", "1", "8"}
	if !sameStrings(got, want) {
		t.Fatalf("findStrobogrammatic() = %v, want %v", got, want)
	}
}
