package leetcode

import "testing"

func TestGenerateParenthesisExampleOne(t *testing.T) {
	got := generateParenthesis(3)
	want := []string{"((()))", "(()())", "(())()", "()(())", "()()()"}
	if !sameStrings(got, want) {
		t.Fatalf("generateParenthesis() = %v, want %v", got, want)
	}
}

func TestGenerateParenthesisExampleTwo(t *testing.T) {
	got := generateParenthesis(1)
	want := []string{"()"}
	if !sameStrings(got, want) {
		t.Fatalf("generateParenthesis() = %v, want %v", got, want)
	}
}
