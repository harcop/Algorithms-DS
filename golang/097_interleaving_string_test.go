package leetcode

import "testing"

func TestIsInterleaveExampleOne(t *testing.T) {
	if !isInterleave("aabcc", "dbbca", "aadbbcbcac") {
		t.Fatal("isInterleave() = false, want true")
	}
}

func TestIsInterleaveExampleTwo(t *testing.T) {
	if isInterleave("aabcc", "dbbca", "aadbbbaccc") {
		t.Fatal("isInterleave() = true, want false")
	}
}

func TestIsInterleaveExampleThree(t *testing.T) {
	if !isInterleave("", "", "") {
		t.Fatal("isInterleave() = false, want true")
	}
}
