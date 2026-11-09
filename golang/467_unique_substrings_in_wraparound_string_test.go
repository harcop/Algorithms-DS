package leetcode

import "testing"

func TestFindSubstringInWraproundStringExamples(t *testing.T) {
	if got := findSubstringInWraproundString("a"); got != 1 {
		t.Fatalf("findSubstringInWraproundString(a) = %d, want 1", got)
	}
	if got := findSubstringInWraproundString("cac"); got != 2 {
		t.Fatalf("findSubstringInWraproundString(cac) = %d, want 2", got)
	}
	if got := findSubstringInWraproundString("zab"); got != 6 {
		t.Fatalf("findSubstringInWraproundString(zab) = %d, want 6", got)
	}
}
