package leetcode

import (
	"sort"
	"strings"
	"testing"
)

func TestWordSquaresExample(t *testing.T) {
	got := wordSquares([]string{"area", "lead", "wall", "lady", "ball"})
	want := [][]string{
		{"wall", "area", "lead", "lady"},
		{"ball", "area", "lead", "lady"},
	}
	if !sameSquares(got, want) {
		t.Fatalf("wordSquares() = %v, want %v", got, want)
	}
}

func sameSquares(a, b [][]string) bool {
	if len(a) != len(b) {
		return false
	}
	key := func(square []string) string { return strings.Join(square, ",") }
	sort.Slice(a, func(i, j int) bool { return key(a[i]) < key(a[j]) })
	sort.Slice(b, func(i, j int) bool { return key(b[i]) < key(b[j]) })
	for i := range a {
		if key(a[i]) != key(b[i]) {
			return false
		}
	}
	return true
}
