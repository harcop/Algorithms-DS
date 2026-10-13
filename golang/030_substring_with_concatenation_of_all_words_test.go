package leetcode

import (
	"sort"
	"testing"
)

func TestFindSubstringExampleOne(t *testing.T) {
	got := findSubstring("barfoothefoobarman", []string{"foo", "bar"})
	sort.Ints(got)
	want := []int{0, 9}
	if !sameInts(got, want) {
		t.Fatalf("findSubstring() = %v, want %v", got, want)
	}
}

func TestFindSubstringExampleTwo(t *testing.T) {
	got := findSubstring("wordgoodgoodgoodbestword", []string{"word", "good", "best", "word"})
	if len(got) != 0 {
		t.Fatalf("findSubstring() = %v, want none", got)
	}
}

func TestFindSubstringExampleThree(t *testing.T) {
	got := findSubstring("barfoofoobarthefoobarman", []string{"bar", "foo", "the"})
	sort.Ints(got)
	want := []int{6, 9, 12}
	if !sameInts(got, want) {
		t.Fatalf("findSubstring() = %v, want %v", got, want)
	}
}
