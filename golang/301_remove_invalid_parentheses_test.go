package leetcode

import "testing"

func TestRemoveInvalidParenthesesExampleOne(t *testing.T) {
	got := removeInvalidParentheses("()())()")
	found := false
	for _, text := range got {
		if text == "(())()" {
			found = true
		}
	}
	if !found {
		t.Fatalf("removeInvalidParentheses() = %v, want it to contain (())()", got)
	}
}
