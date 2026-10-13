package leetcode

import (
	"sort"
	"testing"
)

func TestLetterCombinationsExampleOne(t *testing.T) {
	got := letterCombinations("23")
	want := []string{"ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"}
	if !sameStrings(got, want) {
		t.Fatalf("letterCombinations() = %v, want %v", got, want)
	}
}

func TestLetterCombinationsExampleTwo(t *testing.T) {
	got := letterCombinations("")
	if len(got) != 0 {
		t.Fatalf("letterCombinations() = %v, want none", got)
	}
}

func TestLetterCombinationsExampleThree(t *testing.T) {
	got := letterCombinations("2")
	want := []string{"a", "b", "c"}
	if !sameStrings(got, want) {
		t.Fatalf("letterCombinations() = %v, want %v", got, want)
	}
}

func sameStrings(a, b []string) bool {
	if len(a) != len(b) {
		return false
	}
	aa := append([]string(nil), a...)
	bb := append([]string(nil), b...)
	sort.Strings(aa)
	sort.Strings(bb)
	for i := range aa {
		if aa[i] != bb[i] {
			return false
		}
	}
	return true
}
