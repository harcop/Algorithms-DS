package leetcode

import "testing"

func TestRepeatedStringMatchExamples(t *testing.T) {
	if got := repeatedStringMatch("abcd", "cdabcdab"); got != 3 {
		t.Fatalf("repeatedStringMatch() = %d, want 3", got)
	}
	if got := repeatedStringMatch("a", "aa"); got != 2 {
		t.Fatalf("repeatedStringMatch() = %d, want 2", got)
	}
}
