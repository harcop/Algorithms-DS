package leetcode

import "testing"

func TestLongestValidParenthesesExampleOne(t *testing.T) {
	got := longestValidParentheses("(()")
	if got != 2 {
		t.Fatalf("longestValidParentheses() = %d, want 2", got)
	}
}

func TestLongestValidParenthesesExampleTwo(t *testing.T) {
	got := longestValidParentheses(")()())")
	if got != 4 {
		t.Fatalf("longestValidParentheses() = %d, want 4", got)
	}
}

func TestLongestValidParenthesesExampleThree(t *testing.T) {
	got := longestValidParentheses("")
	if got != 0 {
		t.Fatalf("longestValidParentheses() = %d, want 0", got)
	}
}
