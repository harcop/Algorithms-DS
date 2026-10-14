package leetcode

import (
	"sort"
	"testing"
)

func TestPermuteExampleOne(t *testing.T) {
	got := permute([]int{1, 2, 3})
	want := [][]int{
		{1, 2, 3},
		{1, 3, 2},
		{2, 1, 3},
		{2, 3, 1},
		{3, 1, 2},
		{3, 2, 1},
	}
	if !sameIntGroups(got, want) {
		t.Fatalf("permute() = %v, want %v", got, want)
	}
}

func TestPermuteExampleTwo(t *testing.T) {
	got := permute([]int{0, 1})
	if len(got) != 2 {
		t.Fatalf("permute() = %v, want 2 permutations", got)
	}
}

func TestPermuteExampleThree(t *testing.T) {
	got := permute([]int{1})
	want := [][]int{{1}}
	if !sameIntGroups(got, want) {
		t.Fatalf("permute() = %v, want %v", got, want)
	}
}

func sameIntGroups(a, b [][]int) bool {
	if len(a) != len(b) {
		return false
	}
	norm := func(in [][]int) [][]int {
		out := make([][]int, len(in))
		for i, row := range in {
			out[i] = append([]int(nil), row...)
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
		if !sameInts(na[i], nb[i]) {
			return false
		}
	}
	return true
}
