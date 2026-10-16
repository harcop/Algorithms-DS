package leetcode

import "testing"

func TestMinWindowExampleOne(t *testing.T) {
	got := minWindow("ADOBECODEBANC", "ABC")
	if got != "BANC" {
		t.Fatalf("minWindow() = %q, want %q", got, "BANC")
	}
}

func TestMinWindowExampleTwo(t *testing.T) {
	got := minWindow("a", "a")
	if got != "a" {
		t.Fatalf("minWindow() = %q, want %q", got, "a")
	}
}

func TestMinWindowExampleThree(t *testing.T) {
	got := minWindow("a", "aa")
	if got != "" {
		t.Fatalf("minWindow() = %q, want empty", got)
	}
}
