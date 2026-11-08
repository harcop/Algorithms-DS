package leetcode

import "testing"

func TestFindContentChildrenExamples(t *testing.T) {
	if got := findContentChildren([]int{1, 2, 3}, []int{1, 1}); got != 1 {
		t.Fatalf("findContentChildren() = %d, want 1", got)
	}
	if got := findContentChildren([]int{1, 2}, []int{1, 2, 3}); got != 2 {
		t.Fatalf("findContentChildren() = %d, want 2", got)
	}
}
