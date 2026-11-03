package leetcode

import "testing"

func TestCanConstructExamples(t *testing.T) {
	if canConstruct("a", "b") {
		t.Fatal("canConstruct() = true, want false")
	}
	if canConstruct("aa", "ab") {
		t.Fatal("canConstruct() = true, want false")
	}
	if !canConstruct("aa", "aab") {
		t.Fatal("canConstruct() = false, want true")
	}
}
