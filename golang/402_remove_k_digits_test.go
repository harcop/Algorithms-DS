package leetcode

import "testing"

func TestRemoveKdigitsExamples(t *testing.T) {
	if got := removeKdigits("1432219", 3); got != "1219" {
		t.Fatalf("removeKdigits() = %q, want 1219", got)
	}
	if got := removeKdigits("10200", 1); got != "200" {
		t.Fatalf("removeKdigits() = %q, want 200", got)
	}
	if got := removeKdigits("10", 2); got != "0" {
		t.Fatalf("removeKdigits() = %q, want 0", got)
	}
}
