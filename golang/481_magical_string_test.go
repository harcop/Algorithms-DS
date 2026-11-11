package leetcode

import "testing"

func TestMagicalStringExamples(t *testing.T) {
	if got := magicalString(6); got != 3 {
		t.Fatalf("magicalString(6) = %d, want 3", got)
	}
	if got := magicalString(1); got != 1 {
		t.Fatalf("magicalString(1) = %d, want 1", got)
	}
}
