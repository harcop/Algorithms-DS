package leetcode

import "testing"

func TestCheckPerfectNumberExamples(t *testing.T) {
	if !checkPerfectNumber(28) {
		t.Fatal("checkPerfectNumber(28) = false, want true")
	}
	if checkPerfectNumber(7) {
		t.Fatal("checkPerfectNumber(7) = true, want false")
	}
	if checkPerfectNumber(1) {
		t.Fatal("checkPerfectNumber(1) = true, want false")
	}
}
