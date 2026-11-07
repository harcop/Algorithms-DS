package leetcode

import "testing"

func TestFindAnagramsExamples(t *testing.T) {
	if got := findAnagrams("cbaebabacd", "abc"); !sameInts(got, []int{0, 6}) {
		t.Fatalf("findAnagrams(cbaebabacd) = %v, want [0 6]", got)
	}
	if got := findAnagrams("abab", "ab"); !sameInts(got, []int{0, 1, 2}) {
		t.Fatalf("findAnagrams(abab) = %v, want [0 1 2]", got)
	}
	if got := findAnagrams("ab", "abc"); got != nil {
		t.Fatalf("findAnagrams(short) = %v, want none", got)
	}
}
