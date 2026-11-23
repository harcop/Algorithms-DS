package leetcode

import "testing"

func TestCountSubstringsExamples(t *testing.T) {
	if got := countSubstrings("abc"); got != 3 {
		t.Fatalf("countSubstrings(abc) = %d, want 3", got)
	}
	if got := countSubstrings("aaa"); got != 6 {
		t.Fatalf("countSubstrings(aaa) = %d, want 6", got)
	}
}
