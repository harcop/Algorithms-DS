package leetcode

import "testing"

func TestRepeatedSubstringPatternExamples(t *testing.T) {
	if !repeatedSubstringPattern("abab") {
		t.Fatal("repeatedSubstringPattern(abab) = false, want true")
	}
	if repeatedSubstringPattern("aba") {
		t.Fatal("repeatedSubstringPattern(aba) = true, want false")
	}
}
