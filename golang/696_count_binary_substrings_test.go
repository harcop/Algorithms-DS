package leetcode

import "testing"

func TestCountBinarySubstringsExamples(t *testing.T) {
	if got := countBinarySubstrings("00110011"); got != 6 {
		t.Fatalf("countBinarySubstrings(00110011) = %d, want 6", got)
	}
	if got := countBinarySubstrings("10101"); got != 4 {
		t.Fatalf("countBinarySubstrings(10101) = %d, want 4", got)
	}
}
