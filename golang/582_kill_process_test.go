package leetcode

import (
	"sort"
	"testing"
)

func TestKillProcessExamples(t *testing.T) {
	got := killProcess([]int{1, 3, 10, 5}, []int{3, 0, 5, 3}, 5)
	sort.Ints(got)
	if !sameInts(got, []int{5, 10}) {
		t.Fatalf("killProcess(5) = %v, want [5 10]", got)
	}
	got = killProcess([]int{1, 3, 10, 5}, []int{3, 0, 5, 3}, 3)
	sort.Ints(got)
	if !sameInts(got, []int{1, 3, 5, 10}) {
		t.Fatalf("killProcess(3) = %v, want [1 3 5 10]", got)
	}
}
