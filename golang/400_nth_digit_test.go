package leetcode

import "testing"

func TestFindNthDigitExamples(t *testing.T) {
	if got := findNthDigit(3); got != 3 {
		t.Fatalf("findNthDigit(3) = %d, want 3", got)
	}
	if got := findNthDigit(11); got != 0 {
		t.Fatalf("findNthDigit(11) = %d, want 0", got)
	}
}
