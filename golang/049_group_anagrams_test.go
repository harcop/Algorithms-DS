package leetcode

import (
	"sort"
	"testing"
)

func TestGroupAnagramsExampleOne(t *testing.T) {
	got := groupAnagrams([]string{"eat", "tea", "tan", "ate", "nat", "bat"})
	want := [][]string{
		{"bat"},
		{"nat", "tan"},
		{"ate", "eat", "tea"},
	}
	if !sameStringGroups(got, want) {
		t.Fatalf("groupAnagrams() = %v, want %v", got, want)
	}
}

func TestGroupAnagramsExampleTwo(t *testing.T) {
	got := groupAnagrams([]string{""})
	want := [][]string{{""}}
	if !sameStringGroups(got, want) {
		t.Fatalf("groupAnagrams() = %v, want %v", got, want)
	}
}

func sameStringGroups(a, b [][]string) bool {
	if len(a) != len(b) {
		return false
	}
	norm := func(in [][]string) [][]string {
		out := make([][]string, len(in))
		for i, row := range in {
			cp := append([]string(nil), row...)
			sort.Strings(cp)
			out[i] = cp
		}
		sort.Slice(out, func(i, j int) bool {
			for k := 0; k < len(out[i]) && k < len(out[j]); k++ {
				if out[i][k] != out[j][k] {
					return out[i][k] < out[j][k]
				}
			}
			return len(out[i]) < len(out[j])
		})
		return out
	}

	na, nb := norm(a), norm(b)
	for i := range na {
		if len(na[i]) != len(nb[i]) {
			return false
		}
		for j := range na[i] {
			if na[i][j] != nb[i][j] {
				return false
			}
		}
	}
	return true
}
