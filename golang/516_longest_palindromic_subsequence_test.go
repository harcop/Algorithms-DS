package leetcode

import "testing"

func TestLongestPalindromeSubseqExamples(t *testing.T) {
	if got := longestPalindromeSubseq("bbbab"); got != 4 {
		t.Fatalf("longestPalindromeSubseq(bbbab) = %d, want 4", got)
	}
	if got := longestPalindromeSubseq("cbbd"); got != 2 {
		t.Fatalf("longestPalindromeSubseq(cbbd) = %d, want 2", got)
	}
}
