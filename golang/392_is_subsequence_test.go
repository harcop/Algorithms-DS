package leetcode

import "testing"

func TestIsSubsequenceExamples(t *testing.T) {
	if !isSubsequence("abc", "ahbgdc") {
		t.Fatal("isSubsequence() = false, want true")
	}
	if isSubsequence("axc", "ahbgdc") {
		t.Fatal("isSubsequence() = true, want false")
	}
	if !isSubsequence("", "ahbgdc") {
		t.Fatal("isSubsequence() = false, want true")
	}
}
