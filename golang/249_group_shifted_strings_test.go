package leetcode

import "testing"

func TestGroupStringsExampleOne(t *testing.T) {
	got := groupStrings([]string{"abc", "bcd", "acef", "xyz", "az", "ba", "a", "z"})
	want := [][]string{
		{"a", "z"},
		{"abc", "bcd", "xyz"},
		{"acef"},
		{"az", "ba"},
	}
	if !sameStringGroups(got, want) {
		t.Fatalf("groupStrings() = %v, want %v", got, want)
	}
}
