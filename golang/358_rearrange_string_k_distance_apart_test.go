package leetcode

import "testing"

func TestRearrangeStringExamples(t *testing.T) {
	if got := rearrangeString("aabbcc", 3); got != "abcabc" {
		t.Fatalf("rearrangeString() = %q, want abcabc", got)
	}
	if got := rearrangeString("aaabc", 3); got != "" {
		t.Fatalf("rearrangeString() = %q, want empty", got)
	}
	if got := rearrangeString("aaadbbcc", 2); got != "abacabcd" {
		t.Fatalf("rearrangeString() = %q, want abacabcd", got)
	}
}
