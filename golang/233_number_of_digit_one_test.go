package leetcode

import "testing"

func TestCountDigitOneExampleOne(t *testing.T) {
	if got := countDigitOne(13); got != 6 {
		t.Fatalf("countDigitOne() = %d, want 6", got)
	}
}

func TestCountDigitOneExampleTwo(t *testing.T) {
	if got := countDigitOne(0); got != 0 {
		t.Fatalf("countDigitOne() = %d, want 0", got)
	}
}
