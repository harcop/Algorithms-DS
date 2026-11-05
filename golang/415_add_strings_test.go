package leetcode

import "testing"

func TestAddStringsExamples(t *testing.T) {
	if got := addStrings("11", "123"); got != "134" {
		t.Fatalf("addStrings() = %q, want 134", got)
	}
	if got := addStrings("456", "77"); got != "533" {
		t.Fatalf("addStrings() = %q, want 533", got)
	}
	if got := addStrings("0", "0"); got != "0" {
		t.Fatalf("addStrings() = %q, want 0", got)
	}
}
