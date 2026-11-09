package leetcode

import "testing"

func TestEncodeShortestExamples(t *testing.T) {
	if got := encodeShortest("aaa"); got != "aaa" {
		t.Fatalf("encodeShortest(aaa) = %q, want aaa", got)
	}
	if got := encodeShortest("aaaaa"); got != "5[a]" {
		t.Fatalf("encodeShortest(aaaaa) = %q, want 5[a]", got)
	}
	if got := encodeShortest("aaaaaaaaaa"); got != "10[a]" {
		t.Fatalf("encodeShortest(aaaaaaaaaa) = %q, want 10[a]", got)
	}
	if got := encodeShortest("aabcaabcd"); got != "2[aabc]d" {
		t.Fatalf("encodeShortest(aabcaabcd) = %q, want 2[aabc]d", got)
	}
	if got := encodeShortest("abbbabbbcabbbabbbc"); got != "2[2[abbb]c]" {
		t.Fatalf("encodeShortest(abbbabbbcabbbabbbc) = %q, want 2[2[abbb]c]", got)
	}
}
