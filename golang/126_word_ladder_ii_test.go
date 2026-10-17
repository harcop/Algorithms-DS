package leetcode

import (
	"sort"
	"testing"
)

func TestFindLaddersExampleOne(t *testing.T) {
	got := findLadders("hit", "cog", []string{"hot", "dot", "dog", "lot", "log", "cog"})
	want := [][]string{
		{"hit", "hot", "dot", "dog", "cog"},
		{"hit", "hot", "lot", "log", "cog"},
	}
	if !sameStringPaths(got, want) {
		t.Fatalf("findLadders() = %v, want %v", got, want)
	}
}

func TestFindLaddersExampleTwo(t *testing.T) {
	got := findLadders("hit", "cog", []string{"hot", "dot", "dog", "lot", "log"})
	if len(got) != 0 {
		t.Fatalf("findLadders() = %v, want none", got)
	}
}

func sameStringPaths(a, b [][]string) bool {
	if len(a) != len(b) {
		return false
	}
	norm := func(in [][]string) [][]string {
		out := make([][]string, len(in))
		for i, row := range in {
			out[i] = append([]string(nil), row...)
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
