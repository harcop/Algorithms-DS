package leetcode

import "testing"

func TestCheckRecordIIExamples(t *testing.T) {
	if got := checkRecordII(2); got != 8 {
		t.Fatalf("checkRecordII(2) = %d, want 8", got)
	}
	if got := checkRecordII(1); got != 3 {
		t.Fatalf("checkRecordII(1) = %d, want 3", got)
	}
	if got := checkRecordII(10101); got != 183236316 {
		t.Fatalf("checkRecordII(10101) = %d, want 183236316", got)
	}
}
