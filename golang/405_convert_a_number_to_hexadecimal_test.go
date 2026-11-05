package leetcode

import "testing"

func TestToHexExamples(t *testing.T) {
	if got := toHex(26); got != "1a" {
		t.Fatalf("toHex(26) = %q, want 1a", got)
	}
	if got := toHex(-1); got != "ffffffff" {
		t.Fatalf("toHex(-1) = %q, want ffffffff", got)
	}
	if got := toHex(0); got != "0" {
		t.Fatalf("toHex(0) = %q, want 0", got)
	}
}
