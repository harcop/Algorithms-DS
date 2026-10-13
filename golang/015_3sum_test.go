package leetcode

import (
	"sort"
	"testing"
)

func TestThreeSumExampleOne(t *testing.T) {
	got := threeSum([]int{-1, 0, 1, 2, -1, -4})
	want := [][]int{{-1, -1, 2}, {-1, 0, 1}}
	if !sameIntMatrix(got, want) {
		t.Fatalf("threeSum() = %v, want %v", got, want)
	}
}

func TestThreeSumExampleTwo(t *testing.T) {
	got := threeSum([]int{0, 1, 1})
	if len(got) != 0 {
		t.Fatalf("threeSum() = %v, want no triplets", got)
	}
}

func TestThreeSumExampleThree(t *testing.T) {
	got := threeSum([]int{0, 0, 0})
	want := [][]int{{0, 0, 0}}
	if !sameIntMatrix(got, want) {
		t.Fatalf("threeSum() = %v, want %v", got, want)
	}
}

func sameIntMatrix(a, b [][]int) bool {
	if len(a) != len(b) {
		return false
	}
	norm := func(in [][]int) [][]int {
		out := make([][]int, len(in))
		for i, row := range in {
			cp := append([]int(nil), row...)
			sort.Ints(cp)
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
		if !sameInts(na[i], nb[i]) {
			return false
		}
	}
	return true
}
